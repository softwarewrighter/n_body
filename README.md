# N-Body Galaxy Collision Simulation

A high-performance n-body simulation demonstrating galaxy collisions, built with Rust and WebAssembly.

## Features

- Real-time simulation of thousands of gravitationally interacting particles
- Two spiral galaxies on collision course
- WebGL rendering with particle effects
- Interactive controls for particle count, time step, and gravity strength
- Performance monitoring (FPS, frame time)
- Pure Rust implementation compiled to WebAssembly

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

The simulation is optimized for modern browsers and can handle:
- 10,000 particles at 60 FPS
- 50,000 particles at 30 FPS
- 100,000 particles at 10-15 FPS

Performance will vary based on your hardware.

## Architecture

- **Frontend**: Minimal HTML5/CSS with WASM bindings
- **Simulation**: Pure Rust n-body physics engine
- **Rendering**: WebGL with custom shaders
- **Parallelization**: Currently single-threaded (web workers coming in Phase 1.5)

## Next Steps

- Implement Barnes-Hut algorithm for O(n log n) complexity
- Add web workers for multi-threaded physics
- Implement more sophisticated galaxy generation
- Add camera controls (pan, zoom, rotate)
- Optimize WebGL rendering with instancing