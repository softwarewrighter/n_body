# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

N-Body Galaxy Collision Simulation - A high-performance simulation demonstrating galaxy collisions with a multi-threaded Rust server and WebAssembly client architecture.

## Development Commands

### Building and Running
- **`./scripts/dev.sh`** - Build and start development server (recommended)
- **`./scripts/build.sh`** - Build WASM module and server binary only
- **`./scripts/serve.sh`** - Start production server (requires prior build)
- **`./scripts/debug.sh`** - Start with debug logging and diagnostics
- **`./scripts/clean.sh`** - Clean build artifacts (`--all` also removes Cargo.lock)

### Prerequisites
- wasm-pack must be installed: `cargo install wasm-pack`
- Server runs on http://localhost:4000 (configurable in config.toml)

### Testing
No formal test suite is currently implemented. Test by running the simulation in a browser.

## Architecture

This is a Rust workspace with three crates that implement a client-server n-body physics simulation:

### Workspace Structure
- **`server/`** - Multi-threaded Rust server using Actix-web and WebSockets
  - Uses Rayon for parallel physics computation across all CPU cores
  - Serves static files and handles WebSocket connections
  - Main entry: `server/src/main.rs`
  - Key modules: `physics.rs`, `simulation.rs`, `websocket.rs`, `config.rs`

- **`client/`** - WebAssembly client for rendering
  - Compiled to WASM using wasm-pack with `--target web`
  - WebGL rendering with custom shaders in `client/src/shaders/`
  - Main entry: `client/src/lib.rs`
  - Key module: `renderer.rs`

- **`shared/`** - Common data structures and message types
  - Shared between server and client
  - Contains serializable structs for WebSocket communication

### Communication Protocol
- Real-time WebSocket communication with JSON messages
- Client sends `ClientMessage` (config updates, controls)
- Server sends `ServerMessage` (simulation state, stats)
- Message types defined in `shared/src/lib.rs`

### Build Process
- Server builds to `target/release/n_body_server`
- Client builds WASM to `server/pkg/` directory
- Static assets served from `www/` (symlinked to `server/pkg/` in dev mode)

## Configuration

Server configuration in `config.toml` (auto-generated):
- Server port, host, debug mode
- Simulation parameters (particle count, update rate)
- WebSocket settings (heartbeat, timeouts)

Environment variables:
- `RUST_LOG` - Logging level (info, debug, etc.)
- `N_BODY_DEBUG=1` - Enable debug mode
- `RUST_BACKTRACE=1` - Enable backtraces

## Key Dependencies

### Server
- `actix-web` + `actix-web-actors` - HTTP server and WebSocket handling
- `rayon` - Parallel computation for physics
- `nalgebra` - Linear algebra and vector math
- `num_cpus` - CPU core detection for parallelization

### Client
- `wasm-bindgen` - Rust-JavaScript bindings
- `web-sys` - WebAPI bindings (WebGL, WebSocket, Canvas)
- `nalgebra` - Shared math with server

## Performance Characteristics

- Physics computation is CPU-bound and scales with core count
- Network traffic is minimized through FPS throttling
- Client rendering is GPU-bound (WebGL)
- Supports 10K+ particles at 60 FPS on modern hardware