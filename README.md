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
- Interactive controls for particle count, time step, and gravity strength
- Live performance monitoring (FPS, computation time, CPU usage)
- Automatic reconnection on connection loss

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
3. Open http://localhost:8000 in your browser

## Scripts

The project includes several helper scripts in the `scripts/` directory:

- **`dev.sh`** - Build and start development server (recommended for development)
- **`build.sh`** - Build the WASM module only
- **`serve.sh`** - Start the development server (with optional port argument)
- **`clean.sh`** - Clean build artifacts (use `--all` to also remove Cargo.lock)

### Examples

```bash
# Build and serve (development mode)
./scripts/dev.sh

# Build only
./scripts/build.sh

# Serve on a custom port
./scripts/serve.sh 3000

# Clean all build artifacts
./scripts/clean.sh --all
```

## Controls

- **Particle Count**: Adjust the number of particles (1,000 - 100,000)
- **Time Step**: Control simulation speed
- **Gravity Strength**: Adjust gravitational constant
- **Reset**: Reset the simulation with current settings
- **Pause/Resume**: Pause or resume the simulation

## Performance

With server-side computation on a 72-thread workstation:
- 10,000 particles at 60+ FPS
- 100,000 particles at 60 FPS
- 1,000,000 particles at 30+ FPS

Performance scales with CPU cores. The client only needs to render particles.

## Technical Details

- **Server**: Actix-web with WebSocket actors
- **Physics**: Parallel force calculation using Rayon
- **Client**: WebGL rendering with custom shaders
- **Protocol**: JSON messages over WebSocket

## Next Steps (Phase 2)

- Implement Barnes-Hut algorithm for O(n log n) complexity
- Add CUDA GPU acceleration for even more particles
- Implement advanced galaxy generation (different types)
- Add camera controls (pan, zoom, rotate)
- Multiple simultaneous client support