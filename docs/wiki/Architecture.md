# Architecture Overview

This page provides a comprehensive view of the N-Body simulation's architecture, including system design, component interactions, and data flow.

## Table of Contents
- [High-Level Architecture](#high-level-architecture)
- [Workspace Structure](#workspace-structure)
- [Component Architecture](#component-architecture)
- [Deployment Architecture](#deployment-architecture)
- [Data Flow](#data-flow)

## High-Level Architecture

The system follows a client-server architecture with clear separation of concerns:

```mermaid
graph TB
    subgraph "Browser Environment"
        HTML[HTML/CSS UI]
        WASM[WASM Module]
        WebGL[WebGL Context]
        WS_Client[WebSocket Client]
    end

    subgraph "Rust Server Process"
        HTTP[HTTP Server<br/>Actix-Web]
        WS_Server[WebSocket Server<br/>Actix-Web-Actors]
        SimLoop[Simulation Loop]
        SimState[Simulation State<br/>Arc&lt;Mutex&gt;]
        PhysicsEngine[Physics Engine<br/>Rayon Parallel]
        Watchdog[Watchdog Thread]
    end

    HTML -->|User Input| WASM
    WASM <-->|JSON Messages| WS_Client
    WS_Client <-->|WebSocket| WS_Server
    WS_Server <-->|Lock/Update| SimState
    SimLoop -->|Read/Write| SimState
    SimLoop -->|Compute| PhysicsEngine
    Watchdog -.->|Monitor| SimLoop
    HTTP -->|Serve| HTML
    WASM -->|Render| WebGL

    style HTML fill:#e3f2fd
    style WASM fill:#e1f5ff
    style WebGL fill:#e1f5ff
    style WS_Client fill:#e1f5ff
    style HTTP fill:#ffebee
    style WS_Server fill:#ffebee
    style SimLoop fill:#ffe1e1
    style SimState fill:#ffe1e1
    style PhysicsEngine fill:#ffe1e1
    style Watchdog fill:#fff3e0
```

### Key Design Principles

1. **Server-Authoritative Physics**: All physics computations happen on the server for consistency
2. **Client Rendering Only**: Client receives state updates and renders them
3. **Lock-Based Concurrency**: Simulation state protected by `Arc<Mutex<T>>`
4. **Parallel Physics**: Rayon parallelizes O(n²) force calculations across CPU cores
5. **Throttled Updates**: Visual FPS setting reduces network traffic

## Workspace Structure

The project is organized as a Rust workspace with three interdependent crates:

```mermaid
graph LR
    subgraph "Workspace: n_body"
        Server[server<br/>Binary Crate]
        Client[client<br/>Library Crate]
        Shared[shared<br/>Library Crate]
    end

    Server -.->|depends on| Shared
    Client -.->|depends on| Shared

    Server -->|compiles to| ServerBin[n_body_server<br/>Native Binary]
    Client -->|compiles to| WASMPkg[n_body_client.wasm<br/>WebAssembly]

    ServerBin -->|serves| WASMPkg

    style Server fill:#ffcdd2
    style Client fill:#bbdefb
    style Shared fill:#c8e6c9
    style ServerBin fill:#ef9a9a
    style WASMPkg fill:#90caf9
```

### Crate Dependencies

```mermaid
graph TB
    subgraph "server dependencies"
        ActixWeb[actix-web<br/>HTTP server]
        ActixActors[actix-web-actors<br/>WebSocket]
        Rayon[rayon<br/>Parallelism]
        Nalgebra_S[nalgebra<br/>Math]
        Serde_S[serde<br/>Serialization]
    end

    subgraph "client dependencies"
        WasmBindgen[wasm-bindgen<br/>JS bindings]
        WebSys[web-sys<br/>WebAPIs]
        Nalgebra_C[nalgebra<br/>Math]
        Serde_C[serde<br/>Serialization]
    end

    subgraph "shared dependencies"
        Nalgebra_Sh[nalgebra<br/>Math]
        Serde_Sh[serde<br/>Serialization]
    end

    Server -->|uses| ActixWeb
    Server -->|uses| ActixActors
    Server -->|uses| Rayon
    Server -->|uses| Nalgebra_S
    Server -->|uses| Serde_S

    Client -->|uses| WasmBindgen
    Client -->|uses| WebSys
    Client -->|uses| Nalgebra_C
    Client -->|uses| Serde_C

    Shared -->|uses| Nalgebra_Sh
    Shared -->|uses| Serde_Sh

    style ActixWeb fill:#ffe0b2
    style ActixActors fill:#ffe0b2
    style Rayon fill:#ffe0b2
    style WasmBindgen fill:#b3e5fc
    style WebSys fill:#b3e5fc
```

## Component Architecture

### Server Components

```mermaid
graph TB
    subgraph "Server Process"
        Main[main.rs<br/>Initialization]
        Config[config.rs<br/>Configuration Loader]

        subgraph "Web Layer"
            HTTPServer[HTTP Server<br/>Actix-Web]
            WSHandler[WebSocket Handler<br/>SimulationWebSocket]
            Routes[Route Handlers]
        end

        subgraph "Business Logic"
            Sim[simulation.rs<br/>Simulation State]
            Physics[physics.rs<br/>Parallel Computations]
            Watchdog[watchdog.rs<br/>Health Monitor]
        end

        subgraph "Shared Resources"
            AppState[AppState]
            ArcMutex[Arc&lt;Mutex&lt;Simulation&gt;&gt;]
        end
    end

    Main -->|Initializes| Config
    Main -->|Creates| HTTPServer
    Main -->|Creates| ArcMutex
    Main -->|Starts| Watchdog
    HTTPServer -->|Routes to| Routes
    Routes -->|WebSocket Upgrade| WSHandler
    WSHandler <-->|Lock/Unlock| ArcMutex
    ArcMutex -->|Contains| Sim
    Sim -->|Uses| Physics
    WSHandler -->|Updates| Watchdog

    style Main fill:#ffccbc
    style Config fill:#ffccbc
    style HTTPServer fill:#ffab91
    style WSHandler fill:#ff8a65
    style Sim fill:#ef5350
    style Physics fill:#e53935
    style Watchdog fill:#fff59d
```

### Client Components

```mermaid
graph TB
    subgraph "WASM Module"
        LibRs[lib.rs<br/>Client Entry]
        ClientStruct[Client Struct]

        subgraph "Rendering"
            Renderer[renderer.rs<br/>WebGL Engine]
            Shaders[shaders/<br/>GLSL Programs]
        end

        subgraph "Communication"
            WSClient[WebSocket Client]
            MsgHandler[Message Handler]
        end

        subgraph "State"
            CurrentState[Current State<br/>Option&lt;SimulationState&gt;]
            ClientConfig[Client Config]
        end
    end

    subgraph "JavaScript Layer"
        UI[UI Controls]
        JSGlue[JS Glue Code]
    end

    LibRs -->|Creates| ClientStruct
    ClientStruct -->|Contains| WSClient
    ClientStruct -->|Contains| Renderer
    ClientStruct -->|Manages| CurrentState
    ClientStruct -->|Manages| ClientConfig
    WSClient -->|Receives| MsgHandler
    MsgHandler -->|Updates| CurrentState
    CurrentState -->|Provides to| Renderer
    Renderer -->|Uses| Shaders
    UI -->|Calls| JSGlue
    JSGlue -->|Invokes| ClientStruct

    style LibRs fill:#e1f5ff
    style ClientStruct fill:#b3e5fc
    style Renderer fill:#81d4fa
    style Shaders fill:#4fc3f7
    style WSClient fill:#29b6f6
    style UI fill:#c5e1a5
```

## Deployment Architecture

### Build Process Flow

```mermaid
graph LR
    subgraph "Source"
        ServerSrc[server/src/*.rs]
        ClientSrc[client/src/*.rs]
        SharedSrc[shared/src/*.rs]
    end

    subgraph "Build Tools"
        Cargo[cargo build<br/>--release]
        WasmPack[wasm-pack build<br/>--target web]
    end

    subgraph "Artifacts"
        ServerBin[target/release/<br/>n_body_server]
        WASMFiles[server/pkg/<br/>*.wasm, *.js]
    end

    subgraph "Runtime"
        ServerProc[Server Process<br/>:4000]
        StaticFiles[www/<br/>Static Assets]
    end

    ServerSrc -->|compile| Cargo
    SharedSrc -->|compile| Cargo
    ClientSrc -->|compile| WasmPack
    SharedSrc -->|compile| WasmPack

    Cargo -->|produces| ServerBin
    WasmPack -->|produces| WASMFiles

    ServerBin -->|runs| ServerProc
    ServerProc -->|serves| WASMFiles
    ServerProc -->|serves| StaticFiles

    style Cargo fill:#ff6b6b
    style WasmPack fill:#4ecdc4
    style ServerBin fill:#ffe66d
    style WASMFiles fill:#a8e6cf
    style ServerProc fill:#ff6b6b
```

### Runtime Deployment

```mermaid
graph TB
    subgraph "Production Server"
        Binary[n_body_server]
        ConfigFile[config.toml]
        WWW[www/ directory]
        PKG[pkg/ directory]
    end

    subgraph "Client Browser"
        IndexHTML[index.html]
        WASMRuntime[WASM Runtime]
        WebGLCtx[WebGL Context]
    end

    Binary -->|Reads| ConfigFile
    Binary -->|Serves| WWW
    Binary -->|Serves| PKG
    Binary <-->|WebSocket| WASMRuntime

    WWW -->|Loads| IndexHTML
    IndexHTML -->|Initializes| WASMRuntime
    WASMRuntime -->|Creates| WebGLCtx

    style Binary fill:#ffccbc
    style ConfigFile fill:#fff9c4
    style WASMRuntime fill:#b3e5fc
    style WebGLCtx fill:#c5cae9
```

## Data Flow

### Simulation Update Cycle

```mermaid
sequenceDiagram
    participant Timer as Interval Timer
    participant WS as WebSocket Handler
    participant Sim as Simulation State
    participant Physics as Physics Engine
    participant Client as WASM Client
    participant GPU as WebGL Renderer

    loop Every update_rate_ms (default: 16ms)
        Timer->>WS: Trigger Update
        WS->>Sim: Lock & Call step()
        Sim->>Physics: calculate_accelerations_parallel()

        Note over Physics: Rayon parallel processing<br/>across all CPU cores
        Physics-->>Sim: Vec<Acceleration>

        Sim->>Sim: Update velocities & positions<br/>(parallel with Rayon)
        Sim-->>WS: (State, Stats)
        WS->>WS: Check visual_fps throttle

        alt Time for visual update
            WS->>Client: ServerMessage::State
            Client->>Client: Update current_state
            Client->>GPU: render(particles)
            GPU-->>Client: Frame rendered
        end

        alt Every 30 frames
            WS->>Client: ServerMessage::Stats
            Client->>Client: Update UI stats
        end
    end
```

### Configuration Update Flow

```mermaid
sequenceDiagram
    participant User
    participant UI as Web UI
    participant Client as WASM Client
    participant WS as WebSocket
    participant Server as Server Handler
    participant Sim as Simulation

    User->>UI: Adjust particle count
    UI->>Client: set_particle_count(5000)
    Client->>Client: Update local config
    Client->>WS: ClientMessage::UpdateConfig
    WS->>Server: Receive message
    Server->>Sim: Lock & update_config()

    alt Valid configuration
        Sim->>Sim: Validate particle count
        Sim->>Sim: reset() if needed
        Sim-->>Server: Ok(())
        Server->>WS: ServerMessage::Config
        WS->>Client: Updated config
        Client->>UI: Update UI elements
    else Invalid configuration
        Sim-->>Server: Err("Exceeds max particles")
        Server->>WS: ServerMessage::Error
        WS->>Client: Error message
        Client->>User: Alert error
    end
```

## Concurrency Model

### Thread Architecture

```mermaid
graph TB
    subgraph "Main Thread"
        ActixRuntime[Actix Runtime<br/>Async Executor]
    end

    subgraph "Actix Worker Threads"
        Worker1[Worker Thread 1<br/>HTTP Requests]
        Worker2[Worker Thread 2<br/>HTTP Requests]
        WorkerN[Worker Thread N<br/>HTTP Requests]
    end

    subgraph "WebSocket Actors"
        WSActor1[WS Actor 1<br/>Per Connection]
        WSActor2[WS Actor 2<br/>Per Connection]
    end

    subgraph "Rayon Thread Pool"
        Rayon1[Rayon Thread 1]
        Rayon2[Rayon Thread 2]
        RayonN[Rayon Thread N]
    end

    subgraph "Watchdog"
        WatchdogThread[Monitoring Thread]
    end

    subgraph "Shared State"
        ArcMutexSim[Arc&lt;Mutex&lt;Simulation&gt;&gt;]
    end

    ActixRuntime -->|Spawns| Worker1
    ActixRuntime -->|Spawns| Worker2
    ActixRuntime -->|Spawns| WorkerN

    Worker1 -->|Creates| WSActor1
    Worker2 -->|Creates| WSActor2

    WSActor1 <-->|Lock| ArcMutexSim
    WSActor2 <-->|Lock| ArcMutexSim

    ArcMutexSim -->|Computes Physics| Rayon1
    ArcMutexSim -->|Computes Physics| Rayon2
    ArcMutexSim -->|Computes Physics| RayonN

    WatchdogThread -.->|Monitor| ArcMutexSim

    style ActixRuntime fill:#ff6b6b
    style Worker1 fill:#ffa07a
    style Worker2 fill:#ffa07a
    style WorkerN fill:#ffa07a
    style WSActor1 fill:#4ecdc4
    style WSActor2 fill:#4ecdc4
    style Rayon1 fill:#95e1d3
    style Rayon2 fill:#95e1d3
    style RayonN fill:#95e1d3
    style WatchdogThread fill:#ffe66d
    style ArcMutexSim fill:#c7ecee
```

## Performance Considerations

### Computational Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Physics Calculation | O(n²) | All pairs of particles |
| Parallel Speedup | ~O(n²/c) | c = number of CPU cores |
| State Serialization | O(n) | Linear in particle count |
| Rendering | O(n) | One vertex per particle |
| WebSocket Transfer | O(n) | Network bandwidth limited |

### Bottleneck Analysis

```mermaid
graph LR
    subgraph "Performance Bottlenecks"
        CPU[CPU: Physics O(n²)<br/>Dominant at high particle counts]
        Network[Network: State Transfer<br/>Minimal due to throttling]
        GPU[GPU: WebGL Rendering<br/>Rarely bottlenecks]
        Memory[Memory: State Clone<br/>O(n) per frame]
    end

    ParticleCount[Particle Count]

    ParticleCount -->|Quadratic Impact| CPU
    ParticleCount -->|Linear Impact| Network
    ParticleCount -->|Linear Impact| GPU
    ParticleCount -->|Linear Impact| Memory

    style CPU fill:#ff6b6b
    style Network fill:#4ecdc4
    style GPU fill:#95e1d3
    style Memory fill:#ffe66d
    style ParticleCount fill:#c7ecee
```

## Related Pages

- **[Communication Protocol](Communication-Protocol)** - Detailed message protocol
- **[Server Components](Server-Components)** - Deep dive into server architecture
- **[Client Components](Client-Components)** - Deep dive into client architecture
- **[Performance Tuning](Performance-Tuning)** - Optimization strategies

---

[← Back to Home](Home)
