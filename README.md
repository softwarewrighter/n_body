# N-Body Galaxy Collision Simulation

A high-performance n-body simulation demonstrating galaxy collisions, powered by a multi-threaded Rust server and WebAssembly client.

## Architecture

- **Server**: Multi-threaded Rust server using all CPU cores for physics computation
- **Client**: Thin WebAssembly client for rendering only
- **Communication**: WebSocket for real-time state updates
- **Performance**: Scales to millions of particles using parallel computation

## Features

- Server-side parallel physics computation using Rayon
- Real-time simulation of gravitationally interacting particles
- Two spiral galaxies on collision course
- WebGL rendering with particle effects
- Interactive controls:
  - Logarithmic particle count slider (1K - 1M particles)
  - Time step (physics speed)
  - Visual FPS (rendering speed, independent of physics)
  - Gravity strength
  - Zoom level with camera controls
  - Arrow keys for camera movement
- Live performance monitoring (FPS, computation time, CPU usage)
- Automatic reconnection on connection loss
- Debug mode with comprehensive logging

## Prerequisites

- Rust (latest stable)
- wasm-pack (`cargo install wasm-pack`)
- A modern web browser with WebGL support

## Quick Start

1. Clone the repository
2. Build and run in development mode:
   ```bash
   ./scripts/dev.sh
   ```
3. Open http://localhost:4000 in your browser

## Configuration

The server uses `config.toml` for configuration (auto-generated on first run):

```toml
[server]
port = 4000          # Server port (change if 4000 is in use)
host = "0.0.0.0"     # Bind address
debug = false        # Enable debug mode (or use N_BODY_DEBUG=1)

[simulation]
default_particles = 3000    # Starting particle count
update_rate_ms = 33        # ~30 FPS physics update rate
stats_frequency = 30       # Send stats every N frames

[websocket]
heartbeat_interval_sec = 5  # WebSocket ping interval
client_timeout_sec = 10     # Client timeout
```

## Scripts

The project includes several helper scripts in the `scripts/` directory:

- **`dev.sh`** - Build and start development server (recommended for development)
- **`build.sh`** - Build the WASM module only
- **`serve.sh`** - Start the production server
- **`debug.sh`** - Start server with debug logging enabled
- **`clean.sh`** - Clean build artifacts (use `--all` to also remove Cargo.lock)

### Examples

```bash
# Build and serve (development mode)
./scripts/dev.sh

# Build only
./scripts/build.sh

# Start production server
./scripts/serve.sh

# Run with debug logging
./scripts/debug.sh

# Clean all build artifacts
./scripts/clean.sh --all
```

## Controls

- **Particle Count**: Logarithmic slider (1,000 - 1,000,000 particles)
- **Time Step**: Physics simulation speed (0.005 - 0.05)
- **Visual FPS**: Rendering frame rate (10 - 60 FPS)
- **Gravity Strength**: Gravitational constant multiplier
- **Zoom**: Camera zoom level (0.1x - 5.0x)
- **Arrow Keys**: Move camera (↑↓←→)
- **Reset**: Reset simulation and camera
- **Pause/Resume**: Pause or resume the simulation

## Performance

With server-side computation on an 8-core machine:
- 10,000 particles at 60+ FPS
- 100,000 particles at 30+ FPS

Performance scales with:
- CPU cores (server uses all available cores via Rayon)
- Network bandwidth (reduced by FPS throttling)
- Client GPU capability (WebGL rendering)

## Technical Details

- **Server**: Actix-web with WebSocket actors
- **Physics**: Parallel force calculation using Rayon
- **Client**: WebGL rendering with custom shaders
- **Protocol**: JSON messages over WebSocket

## Development Status

### Phase 1: CPU Parallelism ✅ Complete
- Multi-threaded server with Rayon parallelization
- WebSocket client-server architecture
- Interactive controls and real-time visualization
- Performance optimizations for data transfer

### Phase 2: GPU Acceleration (Planned)
- CUDA implementation for massive particle counts
- Barnes-Hut algorithm for O(n log n) complexity
- Advanced galaxy generation (different types)
- 3D camera controls (rotation)
- Multiple simultaneous client support