# Configuration

Complete guide to configuring the N-Body simulation server and runtime behavior.

## Table of Contents
- [Configuration File](#configuration-file)
- [Environment Variables](#environment-variables)
- [Runtime Configuration](#runtime-configuration)
- [Tuning Guide](#tuning-guide)

## Configuration File

### config.toml Location

The configuration file `config.toml` is located in the project root and is **auto-generated** on first run if it doesn't exist.

```mermaid
graph TB
    Start[Server Starts]
    CheckFile{config.toml<br/>exists?}
    LoadFile[Load config.toml]
    GenerateFile[Generate default<br/>config.toml]
    UseConfig[Use Configuration]

    Start --> CheckFile
    CheckFile -->|Yes| LoadFile
    CheckFile -->|No| GenerateFile
    LoadFile --> UseConfig
    GenerateFile --> UseConfig

    style Start fill:#c8e6c9
    style CheckFile fill:#ffd93d
    style LoadFile fill:#81d4fa
    style GenerateFile fill:#ffaaa5
    style UseConfig fill:#e1f5ff
```

### Default Configuration

```toml
[server]
host = "127.0.0.1"
port = 4000
debug = false

[simulation]
default_particles = 3000
update_rate_ms = 16

[websocket]
heartbeat_interval_sec = 5
client_timeout_sec = 10
```

---

## Configuration Sections

### [server] Section

Controls HTTP server and static file serving.

```mermaid
classDiagram
    class ServerConfig {
        +host: String
        +port: u16
        +debug: bool
    }

    note for ServerConfig "Controls server binding\nand debug output"
```

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | String | "127.0.0.1" | IP address to bind |
| `port` | u16 | 4000 | Port number |
| `debug` | bool | false | Enable debug logging |

**Host Options:**

```mermaid
graph TB
    Host[host setting]

    Localhost["127.0.0.1"<br/>Localhost only]
    AllInterfaces["0.0.0.0"<br/>All interfaces]
    Specific["192.168.1.100"<br/>Specific IP]

    Host -->|Default| Localhost
    Host -->|Network access| AllInterfaces
    Host -->|Custom| Specific

    Localhost -.->|Security| Safe[Safe: Local only]
    AllInterfaces -.->|Caution| Exposed[Exposed to network]
    Specific -.->|Specific| Custom[Custom binding]

    style Localhost fill:#c8e6c9
    style AllInterfaces fill:#ffccbc
    style Specific fill:#81d4fa
```

**Example Configurations:**

```toml
# Development (localhost only)
[server]
host = "127.0.0.1"
port = 4000
debug = true

# Production (all interfaces)
[server]
host = "0.0.0.0"
port = 8080
debug = false

# Custom port
[server]
host = "127.0.0.1"
port = 3000
debug = false
```

---

### [simulation] Section

Initial simulation parameters.

```mermaid
classDiagram
    class SimulationConfig {
        +default_particles: usize
        +update_rate_ms: u64
    }

    note for SimulationConfig "Server-side defaults\nCan be changed at runtime"
```

**Fields:**

| Field | Type | Default | Range | Description |
|-------|------|---------|-------|-------------|
| `default_particles` | usize | 3000 | 1-15000 | Initial particle count |
| `update_rate_ms` | u64 | 16 | 1-1000 | Physics update interval (ms) |

**update_rate_ms and FPS:**

```mermaid
graph LR
    UpdateRate[update_rate_ms]

    FPS16["16ms → 60 FPS"]
    FPS33["33ms → 30 FPS"]
    FPS100["100ms → 10 FPS"]

    UpdateRate -->|Default| FPS16
    UpdateRate -.->|Lower performance| FPS33
    UpdateRate -.->|Very slow| FPS100

    style UpdateRate fill:#ffd93d
    style FPS16 fill:#c8e6c9
    style FPS33 fill:#81d4fa
    style FPS100 fill:#ffccbc
```

**Formula:**

```
FPS = 1000 / update_rate_ms

Examples:
- 16ms → 62.5 FPS
- 20ms → 50 FPS
- 33ms → 30 FPS
```

**Performance Recommendations:**

| Particle Count | Recommended update_rate_ms | Target FPS |
|----------------|---------------------------|------------|
| 1,000-3,000 | 16ms | 60 FPS |
| 3,000-7,000 | 20-25ms | 40-50 FPS |
| 7,000-12,000 | 33ms | 30 FPS |
| 12,000-15,000 | 50ms | 20 FPS |

---

### [websocket] Section

WebSocket connection health monitoring.

```mermaid
classDiagram
    class WebSocketConfig {
        +heartbeat_interval_sec: u64
        +client_timeout_sec: u64
    }

    note for WebSocketConfig "Ping/Pong health checks\nDetect disconnections"
```

**Fields:**

| Field | Type | Default | Range | Description |
|-------|------|---------|-------|-------------|
| `heartbeat_interval_sec` | u64 | 5 | 1-60 | Ping interval |
| `client_timeout_sec` | u64 | 10 | 2-120 | Timeout threshold |

**Heartbeat Mechanism:**

```mermaid
sequenceDiagram
    participant Server
    participant Client

    loop Every heartbeat_interval_sec (5s)
        Server->>Client: Ping
        Client->>Server: Pong

        Note over Server: Update last_heartbeat

        alt Timeout (client_timeout_sec = 10s)
            Server->>Server: No pong for 10s
            Server->>Server: Close connection
            Note over Server: Client considered dead
        end
    end
```

**Relationship:**

```
client_timeout_sec should be > heartbeat_interval_sec × 2

Recommended:
- heartbeat_interval_sec: 5
- client_timeout_sec: 10 (2× interval)
```

**Tuning:**

```toml
# Aggressive detection (fast network)
[websocket]
heartbeat_interval_sec = 2
client_timeout_sec = 5

# Conservative (slow/unreliable network)
[websocket]
heartbeat_interval_sec = 10
client_timeout_sec = 30
```

---

## Environment Variables

Environment variables override config file settings.

```mermaid
graph TB
    ConfigFile[config.toml]
    EnvVars[Environment Variables]
    Runtime[Runtime Configuration]

    EnvVars -->|Override| Runtime
    ConfigFile -->|Default| Runtime

    style EnvVars fill:#ffd93d
    style ConfigFile fill:#81d4fa
    style Runtime fill:#c8e6c9
```

### RUST_LOG

Controls logging verbosity.

**Format:**

```bash
RUST_LOG=<level>
RUST_LOG=<module>=<level>
```

**Levels (least to most verbose):**

```mermaid
graph LR
    Error[error] --> Warn[warn]
    Warn --> Info[info]
    Info --> Debug[debug]
    Debug --> Trace[trace]

    style Error fill:#ff6b6b
    style Warn fill:#ffd93d
    style Info fill:#c8e6c9
    style Debug fill:#81d4fa
    style Trace fill:#e1f5ff
```

**Examples:**

```bash
# Show only errors
RUST_LOG=error ./target/release/n_body_server

# Show info and above (default recommended)
RUST_LOG=info ./target/release/n_body_server

# Show debug logs
RUST_LOG=debug ./target/release/n_body_server

# Per-module logging
RUST_LOG=n_body_server::simulation=debug,n_body_server=info

# All trace (very verbose)
RUST_LOG=trace ./target/release/n_body_server
```

**What Each Level Shows:**

| Level | Shows |
|-------|-------|
| `error` | Critical errors only |
| `warn` | Warnings + errors (performance issues, etc.) |
| `info` | General info + warnings + errors (recommended) |
| `debug` | Debug details + all above (development) |
| `trace` | All logs including function calls (debugging) |

---

### N_BODY_DEBUG

Enables debug mode in simulation.

```bash
N_BODY_DEBUG=1 ./target/release/n_body_server
```

**Effects:**

```mermaid
graph TB
    Debug[N_BODY_DEBUG=1]

    ConfigDebug[config.server.debug = true]
    VerboseLog[Verbose Logging]
    DetailedStats[Detailed Statistics]
    FrameInfo[Per-Frame Info]

    Debug --> ConfigDebug
    ConfigDebug --> VerboseLog
    ConfigDebug --> DetailedStats
    ConfigDebug --> FrameInfo

    style Debug fill:#ffd93d
    style VerboseLog fill:#81d4fa
    style DetailedStats fill:#c8e6c9
    style FrameInfo fill:#e1f5ff
```

**Debug Output Example:**

```
=== DEBUG MODE ENABLED ===
Server config: ServerConfig { host: "127.0.0.1", port: 4000, debug: true }
Simulation config: SimulationConfig { default_particles: 3000, update_rate_ms: 16 }
WebSocket config: WebSocketConfig { heartbeat_interval_sec: 5, client_timeout_sec: 10 }

[DEBUG] WebSocket connection established
[DEBUG] Sending initial config
[DEBUG] Client config updated: 5000 particles
[DEBUG] Frame 100: 5000 particles, 15.2ms computation time
```

---

### RUST_BACKTRACE

Enable stack traces on panics.

```bash
# Show backtrace on crash
RUST_BACKTRACE=1 ./target/release/n_body_server

# Full backtrace (all frames)
RUST_BACKTRACE=full ./target/release/n_body_server
```

**Use Cases:**

- Debugging crashes
- Investigating panics
- Development troubleshooting

---

## Runtime Configuration

Configuration that can be changed while the server is running (via WebSocket messages).

### Client-Configurable Parameters

```mermaid
graph TB
    Client[Client UI]

    ParticleCount[Particle Count]
    TimeStep[Time Step]
    Gravity[Gravity Strength]
    VisualFPS[Visual FPS]
    ZoomLevel[Zoom Level]
    DebugMode[Debug Mode]

    Client -->|UpdateConfig| ParticleCount
    Client -->|UpdateConfig| TimeStep
    Client -->|UpdateConfig| Gravity
    Client -->|UpdateConfig| VisualFPS
    Client -->|UpdateConfig| ZoomLevel
    Client -->|UpdateConfig| DebugMode

    ParticleCount -.->|Validated| Server[Server]
    TimeStep -.->|Applied| Server
    Gravity -.->|Applied| Server
    VisualFPS -.->|Applied| Server
    ZoomLevel -.->|Applied| Server
    DebugMode -.->|Applied| Server

    style Client fill:#e1f5ff
    style Server fill:#ffccbc
```

**Adjustable Parameters:**

| Parameter | Type | Runtime Changeable | Requires Reset |
|-----------|------|-------------------|----------------|
| `particle_count` | usize | ✅ Yes | ✅ Yes |
| `time_step` | f32 | ✅ Yes | ❌ No |
| `gravity_strength` | f32 | ✅ Yes | ❌ No |
| `visual_fps` | u32 | ✅ Yes | ❌ No |
| `zoom_level` | f32 | ✅ Yes | ❌ No |
| `debug` | bool | ✅ Yes | ❌ No |

**Reset Behavior:**

```mermaid
graph LR
    ChangeParticles[Change particle_count]
    Validate[Server Validates]
    Reset[Reset Simulation]
    Regenerate[Regenerate Galaxies]
    SendState[Send New State]

    ChangeParticles --> Validate
    Validate -->|Valid| Reset
    Reset --> Regenerate
    Regenerate --> SendState

    style ChangeParticles fill:#81d4fa
    style Validate fill:#ffd93d
    style Reset fill:#ff6b6b
    style Regenerate fill:#c8e6c9
    style SendState fill:#e1f5ff
```

---

## Tuning Guide

### Performance Tuning

**Goal: Maximize FPS at target particle count**

```mermaid
graph TB
    Goal[Target: 60 FPS<br/>with N particles]

    subgraph "Tune These"
        UpdateRate[update_rate_ms]
        VisualFPS[visual_fps]
        ParticleCount[particle_count]
    end

    subgraph "Monitor These"
        CompTime[computation_time_ms]
        ActualFPS[fps]
        CPUUsage[cpu_usage]
    end

    Goal --> UpdateRate
    Goal --> VisualFPS
    Goal --> ParticleCount

    UpdateRate -.->|Affects| CompTime
    VisualFPS -.->|Affects| CPUUsage
    ParticleCount -.->|Affects| CompTime
    ParticleCount -.->|Affects| ActualFPS

    style Goal fill:#c8e6c9
    style CompTime fill:#ffd93d
    style ActualFPS fill:#81d4fa
```

**Tuning Process:**

1. **Start with defaults**
   ```toml
   [simulation]
   default_particles = 3000
   update_rate_ms = 16
   ```

2. **Increase particles gradually**
   - Monitor `computation_time_ms` in stats
   - Target: < 100ms per frame
   - If exceeded: reduce particles or increase `update_rate_ms`

3. **Adjust visual FPS**
   - Higher visual_fps = more bandwidth
   - Lower visual_fps = smoother network performance
   - Recommended: 20-30 FPS for most cases

4. **Fine-tune update rate**
   - 16ms (60 FPS): Ideal for < 5K particles
   - 33ms (30 FPS): Good for 5K-10K particles
   - 50ms (20 FPS): Works for 10K-15K particles

---

### Network Tuning

**Goal: Minimize bandwidth while maintaining smooth visuals**

```mermaid
graph TB
    Bandwidth[Reduce Bandwidth]

    LowerFPS[Lower visual_fps]
    FewerParticles[Reduce particle_count]

    Bandwidth --> LowerFPS
    Bandwidth --> FewerParticles

    LowerFPS -.->|Effect| Less[Fewer State messages]
    FewerParticles -.->|Effect| Smaller[Smaller State messages]

    Less --> Save[Bandwidth Saved]
    Smaller --> Save

    style Bandwidth fill:#ffd93d
    style LowerFPS fill:#81d4fa
    style FewerParticles fill:#ff6b6b
    style Save fill:#c8e6c9
```

**Bandwidth Calculation:**

```
Bandwidth ≈ particle_count × 40 bytes × visual_fps

Examples:
- 3000 particles @ 30 FPS = 3.6 MB/s
- 5000 particles @ 30 FPS = 6.0 MB/s
- 5000 particles @ 15 FPS = 3.0 MB/s
```

**Recommendations:**

| Network | visual_fps | particle_count |
|---------|-----------|----------------|
| Local (localhost) | 60 | 15000 |
| LAN (fast) | 30-60 | 10000 |
| WiFi (good) | 20-30 | 5000 |
| Slow network | 10-15 | 3000 |

---

### Debug Performance

**Tip:** Disable debug mode in production for better performance.

```toml
[server]
debug = false  # Better performance
```

**Debug Mode Overhead:**

```mermaid
graph LR
    DebugOff[debug = false] -->|Fast| Performance[Better Performance]
    DebugOn[debug = true] -->|Slower| Logging[More Logging Overhead]

    Performance -.->|Savings| CPU[Less CPU usage]
    Performance -.->|Savings| Memory[Less memory]

    Logging -.->|Cost| CPUCost[CPU for formatting]
    Logging -.->|Cost| IOCost[I/O for writing logs]

    style DebugOff fill:#c8e6c9
    style DebugOn fill:#ffccbc
    style Performance fill:#81d4fa
```

---

## Configuration Examples

### High Performance (Local Development)

```toml
[server]
host = "127.0.0.1"
port = 4000
debug = true

[simulation]
default_particles = 5000
update_rate_ms = 16  # 60 FPS

[websocket]
heartbeat_interval_sec = 5
client_timeout_sec = 10
```

**With environment:**

```bash
RUST_LOG=info ./scripts/dev.sh
```

---

### Production (Network Server)

```toml
[server]
host = "0.0.0.0"
port = 8080
debug = false

[simulation]
default_particles = 3000
update_rate_ms = 20  # 50 FPS

[websocket]
heartbeat_interval_sec = 10
client_timeout_sec = 30
```

**With environment:**

```bash
RUST_LOG=warn ./target/release/n_body_server
```

---

### Low-End Hardware

```toml
[server]
host = "127.0.0.1"
port = 4000
debug = false

[simulation]
default_particles = 1000
update_rate_ms = 33  # 30 FPS

[websocket]
heartbeat_interval_sec = 5
client_timeout_sec = 10
```

**Client settings:**

- visual_fps: 15-20
- particle_count: 1000-2000

---

## Troubleshooting

### Issue: Config changes not applied

**Cause:** Config file cached.

**Solution:** Restart server after editing `config.toml`.

```bash
# Stop server (Ctrl+C)
# Edit config.toml
# Restart
./scripts/serve.sh
```

---

### Issue: Port already in use

**Error:**

```
Error: Address already in use (os error 98)
```

**Solution:** Change port in config or kill existing process.

```toml
[server]
port = 4001  # Different port
```

```bash
# Or kill process on port 4000
lsof -ti:4000 | xargs kill -9
```

---

## Related Pages

- **[Development Guide](Development-Guide)** - Build and run instructions
- **[Server Components](Server-Components)** - Server architecture details
- **[Architecture Overview](Architecture)** - System design

---

[← Back to Home](Home)
