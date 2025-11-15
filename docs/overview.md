# N-Body Galaxy Collision Simulation - Project Overview

**Last Updated**: 2025-11-14T19:50:56Z

## Purpose

The N-Body Galaxy Collision Simulation is a high-performance physics simulation that demonstrates gravitational interactions between thousands to millions of particles representing stars in colliding galaxies. The project serves as both a technical showcase for parallel computing architectures and an educational visualization tool for understanding gravitational dynamics.

### Key Objectives
- **Performance**: Leverage modern parallel computing (CPU multi-threading now, GPU acceleration planned) to simulate massive particle systems in real-time
- **Visualization**: Provide an interactive, visually compelling demonstration of galaxy collision physics
- **Architecture**: Demonstrate client-server separation with WebAssembly for thin-client rendering
- **Scalability**: Support particle counts ranging from 1,000 to 1,000,000+ with adaptive performance

## Current Status

### Phase 1: CPU Parallelization - ✅ COMPLETE

The project has successfully completed Phase 1, establishing a robust foundation with all core features implemented and tested.

#### Implemented Features
- **Multi-threaded Rust Server**
  - Actix-web framework with WebSocket support
  - Rayon-based parallel physics computation across all CPU cores
  - Real-time simulation state updates at configurable rates
  - Comprehensive debug mode and logging infrastructure

- **WebAssembly Client**
  - Thin client architecture (rendering only, no physics)
  - WebGL particle rendering with custom shaders
  - Real-time WebSocket communication with automatic reconnection
  - Interactive controls and responsive camera system

- **Galaxy Simulation**
  - Two spiral galaxies initialized on collision trajectory
  - Realistic particle distribution with orbital velocities
  - Color-coded particles for visual enhancement
  - Direct O(n²) gravitational force calculation

- **Interactive Controls**
  - Logarithmic particle count slider (1K - 1M particles)
  - Adjustable time step (simulation speed)
  - Visual FPS control (independent of physics update rate)
  - Gravity strength modification
  - Camera zoom and keyboard-based movement
  - Pause/resume and reset functionality

- **Development Infrastructure**
  - Comprehensive build scripts (`dev.sh`, `build.sh`, `serve.sh`, `debug.sh`, `clean.sh`)
  - TOML-based configuration with sensible defaults
  - Automatic config generation on first run
  - Cross-platform support (tested on macOS and Linux)

#### Performance Achievements
- **10,000 particles**: 60+ FPS
- **100,000 particles**: 30+ FPS
- Linear scaling with CPU core count via Rayon parallelization
- Network optimization via FPS throttling to prevent data flooding
- Smooth WebGL rendering on modern hardware

#### Recent Improvements (Last 10 Commits)
- Added CLAUDE.md for AI assistant guidance
- Fixed WebSocket performance issues causing UI hangs
- Resolved type inference errors in config loading
- Implemented comprehensive debug mode support
- Enhanced camera controls and simulation responsiveness
- Improved UX with better control responsiveness
- Optimized directory structure and routing

### Technical Architecture

#### Workspace Structure
This is a Rust workspace with three crates:

1. **`server/`** - Multi-threaded Actix-web server
   - Physics computation using Rayon for parallelization
   - WebSocket handlers for client communication
   - Configuration management and logging
   - Static file serving

2. **`client/`** - WebAssembly rendering client
   - Compiled to WASM with wasm-pack (`--target web`)
   - WebGL rendering pipeline with custom shaders
   - WebSocket client for state updates
   - User input handling and camera controls

3. **`shared/`** - Common data structures
   - Particle and simulation state definitions
   - Message types for client-server communication
   - Serializable structs (serde)

#### Communication Protocol
- WebSocket-based real-time communication
- JSON serialization for messages
- Heartbeat mechanism for connection health monitoring
- Automatic reconnection on connection loss
- Separate channels for simulation state and statistics

#### Build Process
- WASM client builds to `server/pkg/` directory
- Server binary builds to `target/release/n_body_server`
- Static assets served from `www/` (symlinked in dev mode)
- Single command development workflow via `./scripts/dev.sh`

## Possible Next Steps

### Phase 2: GPU Acceleration (Planned)

The next major development phase focuses on implementing CUDA GPU acceleration to achieve 1M+ particle simulations at high frame rates.

#### Short-term Goals (Phase 2.1-2.2)
1. **CUDA Environment Setup**
   - Install CUDA toolkit and configure build environment
   - Integrate cudarc Rust bindings
   - Implement basic CPU-GPU memory transfer
   - Add GPU backend option with CPU fallback

2. **Basic GPU Kernels**
   - Port O(n²) force calculation to CUDA kernel
   - Implement particle integration kernel (Velocity Verlet)
   - Establish persistent GPU memory management
   - Benchmark GPU vs CPU performance

#### Medium-term Goals (Phase 2.3-2.4)
3. **Memory and Algorithm Optimization**
   - Implement tiled force calculation with shared memory
   - Convert to Structure of Arrays (SoA) memory layout
   - Tune kernel occupancy and register usage
   - Add multi-stream execution for overlap

4. **Advanced GPU Features**
   - Implement Barnes-Hut octree algorithm for O(n log n) complexity
   - Experiment with mixed precision (FP16/FP32)
   - Explore unified memory and zero-copy techniques
   - Design multi-GPU architecture (stretch goal)

#### Long-term Goals (Phase 2.5)
5. **Integration and Production**
   - Hybrid CPU-GPU architecture with dynamic selection
   - GPU-specific configuration options
   - Enhanced visualization with compute shaders
   - Comprehensive testing and performance validation

### Other Enhancement Opportunities

#### Features
- Multiple galaxy types (elliptical, irregular, barred spiral)
- 3D camera rotation controls
- Particle collision and merging
- Export simulation data for analysis
- Multiple simultaneous client support
- Configurable initial conditions UI

#### Optimizations
- Spatial sorting for cache locality
- Adaptive time stepping based on particle density
- Level-of-detail rendering for distant particles
- Memory pooling and object reuse

#### Infrastructure
- Comprehensive test suite (unit and integration)
- Performance regression testing
- CI/CD pipeline
- Docker containerization
- Cloud deployment options (GPU instances)

## Performance Targets

### Current (Phase 1 - CPU)
- 10K particles: 60+ FPS ✅
- 100K particles: 30+ FPS ✅

### Planned (Phase 2 - GPU)
- 1M particles: 60 FPS (target)
- 100K particles: 120+ FPS (target)
- 10K particles: 240+ FPS (target)
- GPU utilization: >90% during computation

## Technical Stack

### Backend
- Rust (latest stable)
- Actix-web + actix-web-actors (HTTP server, WebSocket)
- Rayon (parallel computation)
- nalgebra (linear algebra)
- serde (serialization)
- CUDA (planned for Phase 2)

### Frontend
- WebAssembly (wasm-pack)
- WebGL (rendering)
- JavaScript (minimal glue code)

### Development Tools
- Bash scripts for build automation
- TOML configuration
- cargo (Rust build system)
- wasm-pack (WASM compilation)

## Getting Started

### Prerequisites
- Rust (latest stable)
- wasm-pack: `cargo install wasm-pack`
- Modern web browser with WebGL support

### Quick Start
```bash
# Clone the repository
git clone <repository-url>
cd n_body

# Build and run (single command)
./scripts/dev.sh

# Open browser
open http://localhost:4000
```

### Configuration
Server configuration in `config.toml` (auto-generated):
- Port and host settings
- Simulation parameters (particle count, update rate)
- WebSocket timeouts and heartbeat intervals
- Debug mode toggle

## Project Health

### Strengths
- Clean architecture with well-separated concerns
- Comprehensive documentation (README, status, plan, CLAUDE.md)
- Strong foundation for GPU acceleration work
- Flexible build system with development convenience scripts
- Real-time performance monitoring
- Robust error handling and reconnection logic

### Areas for Improvement
- No formal test suite (relying on manual browser testing)
- Limited to direct O(n²) algorithm (quadratic complexity)
- CPU-only implementation limits maximum particle counts
- Single-client architecture (no multi-user support)

### Risk Assessment
- **Technical Debt**: Low - code is well-organized and documented
- **Maintenance**: Low - stable architecture, minimal dependencies
- **Evolution**: High potential - clear path to GPU acceleration
- **Platform**: Cross-platform Rust with WebAssembly ensures broad compatibility

## Documentation

- **README.md**: User-facing documentation, quick start guide
- **docs/status.md**: Detailed Phase 1 completion status
- **docs/plan.md**: Comprehensive Phase 2 GPU implementation plan
- **CLAUDE.md**: Codebase guidance for AI assistant sessions
- **config.toml**: Runtime configuration (auto-generated)

## Repository Information

- **Current Branch**: main
- **Recent Activity**: Active development with focus on stability and UX improvements
- **Modified Files**: docs/plan.md, docs/status.md (this session)
- **Latest Commit**: "Add CLAUDE.md with codebase guidance for future Claude Code sessions"

---

*This overview is maintained as part of a multi-project management system. It provides a high-level summary for project coordination and planning purposes.*
