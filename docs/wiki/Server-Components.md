# Server Components

Detailed documentation of the Rust server architecture, including modules, threading model, and implementation details.

## Table of Contents
- [Overview](#overview)
- [Module Architecture](#module-architecture)
- [Core Modules](#core-modules)
- [Threading and Concurrency](#threading-and-concurrency)
- [Performance Optimizations](#performance-optimizations)

## Overview

The server is a multi-threaded Rust application built on **Actix-Web** that:
- Serves static files (HTML, WASM, JavaScript)
- Handles WebSocket connections for real-time communication
- Computes n-body physics using parallel processing
- Monitors system health with a watchdog thread

```mermaid
graph TB
    subgraph "Server Binary: n_body_server"
        Main[main.rs<br/>Entry Point]

        subgraph "Web Layer"
            HTTP[HTTP Server<br/>Actix-Web]
            WS[websocket.rs<br/>WebSocket Actor]
        end

        subgraph "Business Logic"
            Sim[simulation.rs<br/>State Management]
            Physics[physics.rs<br/>Future]
            Config[config.rs<br/>Configuration]
            Watch[watchdog.rs<br/>Health Monitor]
        end

        subgraph "External"
            Static[www/<br/>Static Files]
            ConfigFile[config.toml]
        end
    end

    Main -->|Initialize| Config
    Main -->|Create| HTTP
    Main -->|Spawn| Watch
    Config -->|Load| ConfigFile
    HTTP -->|Serve| Static
    HTTP -->|Upgrade| WS
    WS <-->|Lock/Update| Sim
    Sim -->|Compute| Physics
    WS -->|Update| Watch

    style Main fill:#ffccbc
    style HTTP fill:#ff8a65
    style WS fill:#ff6e40
    style Sim fill:#d32f2f
    style Config fill:#ffd54f
    style Watch fill:#fff59d
```

## Module Architecture

### File Structure

```
server/
├── src/
│   ├── main.rs           # Application entry point, HTTP server setup
│   ├── config.rs         # Configuration loading and validation
│   ├── simulation.rs     # Simulation state and physics orchestration
│   ├── physics.rs        # Physics computations (placeholder/future)
│   ├── websocket.rs      # WebSocket actor implementation
│   └── watchdog.rs       # Health monitoring thread
├── Cargo.toml            # Dependencies and build configuration
└── pkg/                  # WASM output directory (generated)
```

### Module Dependencies

```mermaid
graph TB
    Main[main.rs]
    Config[config.rs]
    Sim[simulation.rs]
    Physics[physics.rs]
    WS[websocket.rs]
    Watch[watchdog.rs]

    Main --> Config
    Main --> WS
    Main --> Sim
    Main --> Watch
    WS --> Sim
    WS --> Watch
    WS --> Config
    Sim --> Physics
    Sim --> Config

    style Main fill:#ff6b6b
    style Config fill:#feca57
    style Sim fill:#ff9ff3
    style WS fill:#48dbfb
    style Watch fill:#1dd1a1
```

## Core Modules

### main.rs - Application Entry Point

**Purpose**: Initialize the server, configure threading, and start the HTTP server.

**Key Responsibilities:**
- Load configuration from `config.toml`
- Initialize Rayon thread pool
- Create shared simulation state (`Arc<Mutex<Simulation>>`)
- Start watchdog monitoring thread
- Configure and start Actix-Web HTTP server

**Code Structure:**

```mermaid
graph TB
    Start[Application Start]
    LoadConfig[Load Config<br/>config::Config::load]
    InitLogger[Initialize Logger<br/>env_logger]
    InitRayon[Initialize Rayon<br/>ThreadPoolBuilder]
    CreateSim[Create Simulation<br/>Arc&lt;Mutex&lt;Simulation&gt;&gt;]
    StartWatch[Start Watchdog<br/>10s timeout]
    CreateAppState[Create AppState]
    StartHTTP[Start HTTP Server<br/>Actix-Web]

    Start --> LoadConfig
    LoadConfig --> InitLogger
    InitLogger --> InitRayon
    InitRayon --> CreateSim
    CreateSim --> StartWatch
    StartWatch --> CreateAppState
    CreateAppState --> StartHTTP

    style Start fill:#a8e6cf
    style LoadConfig fill:#ffd3b6
    style CreateSim fill:#ffaaa5
    style StartHTTP fill:#ff8b94
```

**Key Code Sections:**

| Section | Purpose | Location |
|---------|---------|----------|
| Configuration | Load `config.toml` | Line 52 |
| Rayon Init | Configure thread pool | Lines 64-68 |
| Simulation Init | Create shared state | Lines 70-73 |
| Watchdog | Start monitoring | Lines 76-78 |
| HTTP Server | Bind and serve | Lines 93-111 |

**Routes:**

```rust
App::new()
    .route("/", web::get().to(index))           // Serve index.html
    .route("/ws", web::get().to(ws_index))      // WebSocket endpoint
    .service(actix_files::Files::new("/", "www")) // Static files
```

---

### config.rs - Configuration Management

**Purpose**: Load, validate, and provide configuration for all server components.

**Configuration Structure:**

```mermaid
graph TB
    ConfigFile[config.toml]

    subgraph "Config Struct"
        Server[ServerConfig<br/>host, port, debug]
        Sim[SimulationConfig<br/>particles, update_rate]
        WS[WebSocketConfig<br/>heartbeat, timeout]
    end

    ConfigFile -->|Load| Server
    ConfigFile -->|Load| Sim
    ConfigFile -->|Load| WS

    style ConfigFile fill:#ffd93d
    style Server fill:#fcbad3
    style Sim fill:#a8d8ea
    style WS fill:#aa96da
```

**Configuration Fields:**

```rust
pub struct Config {
    pub server: ServerConfig,
    pub simulation: SimulationConfig,
    pub websocket: WebSocketConfig,
}

pub struct ServerConfig {
    pub host: String,              // Default: "127.0.0.1"
    pub port: u16,                 // Default: 4000
    pub debug: bool,               // Default: false
}

pub struct SimulationConfig {
    pub default_particles: usize,  // Default: 3000
    pub update_rate_ms: u64,       // Default: 16 (60 FPS)
}

pub struct WebSocketConfig {
    pub heartbeat_interval_sec: u64,  // Default: 5
    pub client_timeout_sec: u64,      // Default: 10
}
```

**Default Values:**

| Setting | Default | Range |
|---------|---------|-------|
| `server.host` | "127.0.0.1" | Any valid IP |
| `server.port` | 4000 | 1024-65535 |
| `server.debug` | false | true/false |
| `simulation.default_particles` | 3000 | 1-15000 |
| `simulation.update_rate_ms` | 16 | 1-1000 |
| `websocket.heartbeat_interval_sec` | 5 | 1-60 |
| `websocket.client_timeout_sec` | 10 | 2-120 |

**Auto-generation**: If `config.toml` doesn't exist, default values are written to disk.

---

### simulation.rs - Simulation State Management

**Purpose**: Manage simulation state, orchestrate physics updates, and handle configuration changes.

**State Diagram:**

```mermaid
stateDiagram-v2
    [*] --> Initializing
    Initializing --> Running: new()

    Running --> Paused: set_paused(true)
    Paused --> Running: set_paused(false)

    Running --> Resetting: reset() or config change
    Paused --> Resetting: reset() or config change
    Resetting --> Running: Regenerate particles

    Running --> Running: step() [if not paused]
    Paused --> Paused: step() [no physics update]

    note right of Running
        Physics computation active
        Particles updated each frame
    end note

    note right of Paused
        Physics frozen
        Rendering continues
    end note
```

**Core Data Structure:**

```rust
pub struct Simulation {
    particles: Vec<Particle>,           // Current particle state
    config: SimulationConfig,           // Simulation parameters
    sim_time: f32,                      // Elapsed simulation time
    frame_number: u64,                  // Sequential frame counter
    is_paused: bool,                    // Pause state
    last_computation_time: f32,         // Last frame time (ms)
    consecutive_slow_frames: u32,       // Performance monitoring
}
```

**Key Methods:**

```mermaid
classDiagram
    class Simulation {
        +new(config, debug) Simulation
        +reset() void
        +update_config(config) Result
        +set_paused(bool) void
        +step() (State, Stats)
        +get_config() &Config
        -calculate_accelerations_parallel() Vec~Acceleration~
        -estimate_cpu_usage() f32
    }

    class Particle {
        +position Point3
        +velocity Vector3
        +mass f32
        +color [f32; 4]
    }

    Simulation --> "*" Particle : contains
```

**step() Method Flow:**

```mermaid
graph TB
    Start[step called]
    CheckPaused{is_paused?}
    CalcAccel[calculate_accelerations_parallel<br/>O(n²) Rayon parallel]
    UpdateParticles[Update particles<br/>Parallel with Rayon]
    IncrementTime[Increment sim_time,<br/>frame_number]
    CalcStats[Calculate Stats]
    CheckPerf{Computation time<br/>> 200ms?}
    LogWarning[Log performance warning]
    Return[Return State, Stats]

    Start --> CheckPaused
    CheckPaused -->|Not Paused| CalcAccel
    CheckPaused -->|Paused| CalcStats
    CalcAccel --> UpdateParticles
    UpdateParticles --> IncrementTime
    IncrementTime --> CalcStats
    CalcStats --> CheckPerf
    CheckPerf -->|Yes| LogWarning
    CheckPerf -->|No| Return
    LogWarning --> Return

    style Start fill:#a8e6cf
    style CalcAccel fill:#ff6b6b
    style UpdateParticles fill:#ff8787
    style CheckPerf fill:#feca57
    style LogWarning fill:#ff9ff3
    style Return fill:#48dbfb
```

**Physics Update (Parallel):**

```rust
// Calculate accelerations in parallel using Rayon
let accelerations = self.calculate_accelerations_parallel();

// Update particles in parallel
self.particles
    .par_iter_mut()
    .zip(accelerations.par_iter())
    .for_each(|(particle, &acceleration)| {
        particle.velocity += acceleration * self.config.time_step;
        particle.position += particle.velocity * self.config.time_step;
    });
```

**Performance Monitoring:**

The simulation tracks computation time and logs warnings:
- **Single slow frame** (>200ms): Log warning with particle count
- **10 consecutive slow frames**: Log error, suggest reducing particles
- **Automatic**: Counter resets when performance improves

---

### websocket.rs - WebSocket Actor

**Purpose**: Handle WebSocket connections, manage message exchange, and drive simulation updates.

**Actor Architecture:**

```mermaid
graph TB
    subgraph "Actix Actor System"
        ActixRuntime[Actix Runtime]
        WorkerThread[Worker Thread]
        Actor[SimulationWebSocket Actor]
    end

    subgraph "Actor State"
        SimRef[Arc&lt;Mutex&lt;Simulation&gt;&gt;]
        WatchdogRef[Arc&lt;SimulationWatchdog&gt;]
        LastHB[last_heartbeat: Instant]
        LastRender[last_render: Instant]
        LastPhysics[last_physics_update: Instant]
    end

    subgraph "Timers"
        HeartbeatTimer[Heartbeat Interval<br/>5s]
        SimTimer[Simulation Loop<br/>16ms]
    end

    ActixRuntime -->|Spawns| WorkerThread
    WorkerThread -->|Creates| Actor
    Actor -->|Contains| SimRef
    Actor -->|Contains| WatchdogRef
    Actor -->|Contains| LastHB
    Actor -->|Contains| LastRender
    Actor -->|Contains| LastPhysics
    Actor -->|Starts| HeartbeatTimer
    Actor -->|Starts| SimTimer

    style Actor fill:#48dbfb
    style SimRef fill:#ff6b6b
    style HeartbeatTimer fill:#1dd1a1
    style SimTimer fill:#ff9ff3
```

**Lifecycle Hooks:**

```mermaid
sequenceDiagram
    participant Actix as Actix Runtime
    participant Actor as SimulationWebSocket
    participant Client as WebSocket Client

    Actix->>Actor: Create actor
    Actix->>Actor: started(&mut ctx)

    activate Actor
    Actor->>Actor: start_heartbeat(ctx)
    Actor->>Actor: start_simulation_loop(ctx)
    Actor->>Actor: Send initial config
    Actor->>Client: ServerMessage::Config
    deactivate Actor

    Note over Actor,Client: Actor running...

    Actix->>Actor: stopped(&mut ctx)
    activate Actor
    Note over Actor: Cleanup resources
    deactivate Actor
```

**Simulation Loop Timer:**

```mermaid
sequenceDiagram
    participant Timer as Interval Timer
    participant Actor as WebSocket Actor
    participant Sim as Simulation
    participant Client

    loop Every 16ms
        Timer->>Actor: Trigger
        Actor->>Actor: Check elapsed time
        Actor->>Sim: lock().step()
        Sim-->>Actor: (State, Stats)
        Actor->>Actor: Update watchdog

        alt Visual FPS interval (33ms for 30 FPS)
            Actor->>Actor: Serialize State
            Actor->>Client: Send State JSON
        end

        alt Every 30 frames
            Actor->>Actor: Serialize Stats
            Actor->>Client: Send Stats JSON
        end
    end
```

**Message Handling:**

```mermaid
graph TB
    Receive[Receive WebSocket Message]
    CheckType{Message Type}

    Text[Text Message]
    Ping[Ping Message]
    Pong[Pong Message]
    Close[Close Message]
    Error[Protocol Error]

    ParseJSON[Parse JSON]
    CheckMsg{Client Message Type}

    UpdateConfig[Handle UpdateConfig]
    Reset[Handle Reset]
    Pause[Handle Pause]
    Resume[Handle Resume]

    Receive --> CheckType
    CheckType -->|Text| Text
    CheckType -->|Ping| Ping
    CheckType -->|Pong| Pong
    CheckType -->|Close| Close
    CheckType -->|Error| Error

    Text --> ParseJSON
    ParseJSON --> CheckMsg

    CheckMsg -->|UpdateConfig| UpdateConfig
    CheckMsg -->|Reset| Reset
    CheckMsg -->|Pause| Pause
    CheckMsg -->|Resume| Resume

    Ping -->|Send Pong| Pong
    Pong -->|Update heartbeat| Pong
    Close -->|Stop actor| Close
    Error -->|Stop actor| Error

    style Receive fill:#a8e6cf
    style UpdateConfig fill:#ffd93d
    style Reset fill:#ff6b6b
    style Pause fill:#48dbfb
    style Resume fill:#1dd1a1
```

**Key Features:**

1. **Heartbeat Management**: Ping/Pong every 5s, timeout after 10s
2. **Visual FPS Throttling**: Only send State updates at configured FPS
3. **Stats Throttling**: Send Stats every 30 frames to reduce traffic
4. **Error Handling**: Send Error messages back to client
5. **Graceful Shutdown**: Stop actor on close or error

---

### watchdog.rs - Health Monitoring

**Purpose**: Monitor simulation for hangs or performance issues.

**Architecture:**

```mermaid
graph TB
    subgraph "Watchdog Thread"
        Thread[Monitoring Thread]
        LastFrame[Arc&lt;AtomicU64&gt;<br/>last_frame_number]
        CheckTimer[Timer: 10s interval]
    end

    subgraph "WebSocket Actors"
        Actor1[Actor 1]
        Actor2[Actor 2]
        ActorN[Actor N]
    end

    Actor1 -->|heartbeat| LastFrame
    Actor2 -->|heartbeat| LastFrame
    ActorN -->|heartbeat| LastFrame

    CheckTimer -->|Check| Thread
    Thread -->|Read| LastFrame
    Thread -->|Log if stuck| Thread

    style Thread fill:#fff59d
    style LastFrame fill:#ffccbc
    style CheckTimer fill:#ffd54f
```

**Detection Logic:**

```mermaid
sequenceDiagram
    participant Watchdog as Watchdog Thread
    participant Counter as AtomicU64 Counter
    participant Log as Logger

    loop Every 10 seconds
        Watchdog->>Counter: Load current frame
        Watchdog->>Watchdog: Compare to last check

        alt Frame number unchanged
            Watchdog->>Log: ERROR: "Simulation may be hung"
            Note over Watchdog: Computation taking > 10s
        else Frame number increased
            Watchdog->>Watchdog: Update last_checked
            Note over Watchdog: Simulation healthy
        end
    end
```

**Use Cases:**

| Scenario | Detection | Action |
|----------|-----------|--------|
| **Too many particles** | Frame updates stop | Log error after 10s |
| **Deadlock** | Frame counter frozen | Log error after 10s |
| **Infinite loop** | No frame progression | Log error after 10s |
| **Normal operation** | Frame counter increments | No action |

---

## Threading and Concurrency

### Thread Architecture

```mermaid
graph TB
    subgraph "Process: n_body_server"
        MainThread[Main Thread<br/>Actix Runtime Setup]

        subgraph "Actix Worker Threads"
            Worker1[Worker 1<br/>HTTP Requests]
            Worker2[Worker 2<br/>HTTP Requests]
            WorkerN[Worker N<br/>HTTP Requests]
        end

        subgraph "Actix Actor Threads"
            WSActor1[WebSocket Actor 1]
            WSActor2[WebSocket Actor 2]
        end

        subgraph "Rayon Thread Pool"
            Rayon1[Rayon Thread 1<br/>Physics]
            Rayon2[Rayon Thread 2<br/>Physics]
            RayonM[Rayon Thread M<br/>Physics]
        end

        WatchdogThread[Watchdog Thread<br/>Monitoring]

        SharedSim[Arc&lt;Mutex&lt;Simulation&gt;&gt;]
    end

    MainThread -->|Spawns| Worker1
    MainThread -->|Spawns| Worker2
    MainThread -->|Spawns| WorkerN
    MainThread -->|Spawns| WatchdogThread

    Worker1 -->|Creates| WSActor1
    Worker2 -->|Creates| WSActor2

    WSActor1 <-->|Lock/Unlock| SharedSim
    WSActor2 <-->|Lock/Unlock| SharedSim
    WatchdogThread -.->|Monitor| SharedSim

    SharedSim -->|Compute| Rayon1
    SharedSim -->|Compute| Rayon2
    SharedSim -->|Compute| RayonM

    style MainThread fill:#ff6b6b
    style Worker1 fill:#feca57
    style Worker2 fill:#feca57
    style WorkerN fill:#feca57
    style WSActor1 fill:#48dbfb
    style WSActor2 fill:#48dbfb
    style Rayon1 fill:#1dd1a1
    style Rayon2 fill:#1dd1a1
    style RayonM fill:#1dd1a1
    style WatchdogThread fill:#ff9ff3
    style SharedSim fill:#a8e6cf
```

### Concurrency Model

**Lock-Based Synchronization:**

```rust
// Shared simulation state
let simulation = Arc::new(Mutex::new(Simulation::new(&config, debug)));

// WebSocket actor locks for updates
match self.simulation.lock() {
    Ok(mut sim) => {
        let (state, stats) = sim.step();
        // ... send to client
    }
    Err(e) => error!("Failed to lock: {}", e),
}
```

**Parallel Physics with Rayon:**

```rust
// Outer loop parallelized across CPU cores
(0..n)
    .into_par_iter()  // Rayon parallel iterator
    .map(|i| {
        // Calculate acceleration for particle i
        // Inner loop sequential but many i computed in parallel
    })
    .collect()
```

### Lock Contention

**Potential Bottlenecks:**

```mermaid
graph LR
    WS1[WebSocket 1] -->|Want Lock| Mutex[Mutex&lt;Simulation&gt;]
    WS2[WebSocket 2] -->|Want Lock| Mutex
    WS3[WebSocket N] -->|Want Lock| Mutex

    Mutex -->|One at a time| Holder[Current Lock Holder]

    style Mutex fill:#ff6b6b
    style Holder fill:#1dd1a1
    style WS1 fill:#48dbfb
    style WS2 fill:#48dbfb
    style WS3 fill:#48dbfb
```

**Mitigation Strategies:**

1. **Short Critical Sections**: Lock held only during `step()` call
2. **Single Writer Pattern**: Typically one active WebSocket per client
3. **Read-After-Write**: No read-only lock mode (could use RwLock)
4. **Fast Physics**: Rayon parallelism keeps lock time low

---

## Performance Optimizations

### Parallel Physics Computation

**Algorithm: O(n²) All-Pairs Force Calculation**

```mermaid
graph TB
    Start[Start: N particles]
    OuterLoop[Outer Loop: Particles 0..N]
    InnerLoop[Inner Loop: Particles 0..N]
    CalcForce[Calculate force<br/>between i and j]
    Accumulate[Accumulate acceleration<br/>for particle i]
    Return[Return Vec&lt;Acceleration&gt;]

    Start --> OuterLoop
    OuterLoop -->|Rayon: Parallel| InnerLoop
    InnerLoop -->|For each j| CalcForce
    CalcForce --> Accumulate
    Accumulate --> Return

    style Start fill:#a8e6cf
    style OuterLoop fill:#ff6b6b
    style InnerLoop fill:#ff8787
    style CalcForce fill:#feca57
```

**Speedup with Rayon:**

| Cores | Sequential Time | Parallel Time | Speedup |
|-------|----------------|---------------|---------|
| 1 | 100ms | 100ms | 1.0× |
| 4 | 100ms | ~27ms | 3.7× |
| 8 | 100ms | ~14ms | 7.1× |
| 16 | 100ms | ~8ms | 12.5× |

*Note: Speedup varies based on particle count and CPU architecture.*

### Visual FPS Throttling

**Problem**: At high particle counts, sending 60 State updates/second wastes bandwidth.

**Solution**: `visual_fps` setting controls render update rate independently of physics rate.

```rust
// Physics updates: Every 16ms (60 FPS)
// Visual updates: Based on visual_fps (default 30 FPS = 33ms)

let render_interval_ms = 1000 / visual_fps;

if act.last_render.elapsed().as_millis() >= render_interval_ms as u128 {
    act.last_render = Instant::now();
    ctx.text(json);  // Send State update
}
```

**Bandwidth Savings:**

| Visual FPS | Messages/sec | Bandwidth (5K particles) |
|------------|--------------|--------------------------|
| 60 | 60 | ~15 MB/s |
| 30 | 30 | ~7.5 MB/s |
| 15 | 15 | ~3.75 MB/s |

### Particle Limits

**Safety Mechanism:**

```rust
pub const MAX_PARTICLES: usize = 15_000;
pub const MAX_COMPUTATION_TIME_MS: f32 = 200.0;

// In update_config:
if config.particle_count > MAX_PARTICLES {
    return Err(format!(
        "Particle count {} exceeds maximum of {}...",
        config.particle_count, MAX_PARTICLES
    ));
}
```

**Why 15,000?**

- **15,000 particles** = 225 million force calculations per frame
- Target: < 100ms computation time on modern CPUs
- Leaves headroom for UI responsiveness

### State Cloning

**Current Approach:**

```rust
let state = SimulationState {
    particles: self.particles.clone(),  // Full clone every frame
    sim_time: self.sim_time,
    frame_number: self.frame_number,
};
```

**Cost:** O(n) memory allocation and copy per frame.

**Potential Optimizations:**
1. Use `Arc<Vec<Particle>>` to avoid clone
2. Implement delta updates (send only changed particles)
3. Use double buffering pattern

---

## Related Pages

- **[Architecture Overview](Architecture)** - High-level system design
- **[Communication Protocol](Communication-Protocol)** - WebSocket protocol details
- **[Client Components](Client-Components)** - WASM client architecture
- **[Performance Tuning](Performance-Tuning)** - Optimization guide

---

[← Back to Home](Home)
