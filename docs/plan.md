# N-Body Simulation - Phase 2 Implementation Plan

## Phase 2: CUDA GPU Implementation

### Objective
Transform the existing CPU-based n-body simulation to leverage GPU acceleration, targeting 1M+ particles at 60 FPS while maintaining the current client-server architecture.

### GPU Architecture Considerations
- **Target Hardware**: RTX 3060 (3,584 CUDA cores, 12GB VRAM)
- **Warp size**: 32 threads
- **Shared memory**: 48KB per SM
- **Performance Goal**: 1M+ particles at 60 FPS

### Technology Stack

#### CUDA Integration Options
- **Primary**: `cudarc` for safe CUDA bindings in Rust
- **Alternative**: `rust-cuda` for kernel development
- **Build System**: Custom build.rs for CUDA compilation
- **Kernel Language**: CUDA C++ for compute kernels

### Implementation Phases

#### Phase 2.1: CUDA Environment Setup (Week 1)
**Goal**: Establish CUDA development environment and basic GPU communication

**Tasks**:
1. **CUDA Toolkit Installation**
   - Install CUDA 12.x toolkit
   - Configure PATH and environment variables
   - Verify with `nvcc --version` and `nvidia-smi`

2. **Rust CUDA Integration**
   - Add `cudarc` dependency to server Cargo.toml
   - Configure build.rs for CUDA compilation
   - Create minimal "Hello GPU" test

3. **Memory Transfer Test**
   - Implement basic CPU-GPU memory transfers
   - Test with simple array operations
   - Verify performance vs CPU baseline

4. **Integration with Existing Server**
   - Add GPU backend option to simulation
   - Maintain CPU fallback for compatibility
   - Runtime GPU detection and selection

**Deliverables**:
- Working CUDA development environment
- Basic GPU memory allocation/transfer
- Integration points identified in existing codebase

#### Phase 2.2: Basic GPU Kernels (Week 2)
**Goal**: Port core physics computation to GPU with direct O(n²) implementation

**Tasks**:
1. **Force Calculation Kernel**
   ```cuda
   __global__ void calculate_forces_naive(
       float3* positions,
       float3* velocities, 
       float3* forces,
       float* masses,
       int n_particles,
       float softening
   )
   ```
   - Direct particle-to-particle force calculation
   - Simple thread-per-particle mapping
   - Basic gravitational force implementation

2. **Integration Kernel**
   ```cuda
   __global__ void integrate_particles(
       float3* positions,
       float3* velocities,
       float3* forces,
       float dt,
       int n_particles
   )
   ```
   - Velocity Verlet integration scheme
   - Coalesced memory access patterns
   - Position and velocity updates

3. **GPU Memory Management**
   - Persistent GPU buffers for particle data
   - Double-buffering for position updates
   - Efficient CPU-GPU synchronization points

4. **Performance Baseline**
   - Compare GPU vs existing CPU implementation
   - Measure kernel execution times
   - Identify initial bottlenecks

**Deliverables**:
- Working O(n²) GPU implementation
- Performance comparison with CPU version
- Baseline measurements for optimization

#### Phase 2.3: Memory and Algorithm Optimization (Week 3)
**Goal**: Optimize GPU kernels for maximum performance and memory efficiency

**Tasks**:
1. **Tiled Force Calculation**
   ```cuda
   __global__ void calculate_forces_tiled(
       float3* positions,
       float3* forces,
       float* masses,
       int n_particles,
       float softening
   )
   ```
   - Shared memory for particle data blocks
   - Reduced global memory accesses
   - Warp-level optimizations

2. **Memory Layout Optimization**
   - Structure of Arrays (SoA) instead of Array of Structures (AoS)
   - Aligned memory access patterns
   - Minimize memory bandwidth usage

3. **Kernel Occupancy Optimization**
   - Tune block sizes for maximum occupancy
   - Minimize register usage per thread
   - Balance shared memory usage

4. **Multi-Stream Execution**
   - Overlap computation with memory transfers
   - Async kernel launches
   - Pipeline CPU-GPU operations

**Deliverables**:
- Optimized tiled force calculation kernel
- Memory-efficient data structures
- 10x+ performance improvement over baseline

#### Phase 2.4: Advanced GPU Features (Week 4)
**Goal**: Implement advanced algorithms and multi-GPU considerations

**Tasks**:
1. **Barnes-Hut Tree Algorithm** (Stretch Goal)
   - GPU-based octree construction
   - Parallel tree traversal
   - O(n log n) complexity for large particle counts

2. **Adaptive Precision**
   - Mixed precision (FP16/FP32) computation
   - Quality vs performance trade-offs
   - Real-time precision adjustment

3. **Advanced Memory Techniques**
   - Unified Memory experimentation
   - Texture memory for constants
   - Zero-copy memory where applicable

4. **Multi-GPU Preparation**
   - Domain decomposition strategies
   - Inter-GPU communication patterns
   - Load balancing algorithms

**Deliverables**:
- Advanced algorithm implementations
- Scalability analysis and recommendations
- Multi-GPU architecture design

#### Phase 2.5: Integration and Polish (Week 5)
**Goal**: Complete integration with existing system and production readiness

**Tasks**:
1. **Hybrid CPU-GPU Architecture**
   - Dynamic backend selection
   - Graceful fallback to CPU
   - Runtime performance monitoring

2. **Configuration and Controls**
   - GPU-specific configuration options
   - Real-time quality adjustments
   - Performance profiling integration

3. **Enhanced Visualization Pipeline**
   - GPU-direct rendering (avoid CPU roundtrip)
   - Compute shader integration
   - Advanced particle effects

4. **Testing and Validation**
   - Comprehensive test suite
   - Performance regression testing
   - Cross-platform compatibility

**Deliverables**:
- Production-ready GPU implementation
- Complete test coverage
- Performance documentation

### Technical Architecture

#### GPU Kernel Design Patterns

**Force Calculation Kernel**:
```cuda
__global__ void calculate_forces_optimized(
    const float3* __restrict__ positions,
    const float* __restrict__ masses,
    float3* __restrict__ forces,
    int n_particles,
    float softening_sq
) {
    extern __shared__ float3 shared_pos[];
    
    int tid = threadIdx.x;
    int bid = blockIdx.x;
    int gtid = bid * blockDim.x + tid;
    
    float3 my_pos = positions[gtid];
    float3 force = {0.0f, 0.0f, 0.0f};
    
    // Tiled computation using shared memory
    for (int tile = 0; tile < gridDim.x; tile++) {
        // Load tile into shared memory
        int idx = tile * blockDim.x + tid;
        if (idx < n_particles) {
            shared_pos[tid] = positions[idx];
        }
        __syncthreads();
        
        // Compute forces for this tile
        for (int j = 0; j < blockDim.x; j++) {
            float3 r = {
                shared_pos[j].x - my_pos.x,
                shared_pos[j].y - my_pos.y,
                shared_pos[j].z - my_pos.z
            };
            
            float dist_sq = r.x*r.x + r.y*r.y + r.z*r.z + softening_sq;
            float inv_dist = rsqrtf(dist_sq);
            float inv_dist3 = inv_dist * inv_dist * inv_dist;
            
            float mass_j = masses[tile * blockDim.x + j];
            force.x += mass_j * r.x * inv_dist3;
            force.y += mass_j * r.y * inv_dist3;
            force.z += mass_j * r.z * inv_dist3;
        }
        __syncthreads();
    }
    
    forces[gtid] = force;
}
```

#### Memory Management Strategy
- **Persistent GPU Memory**: Keep particle data resident on GPU
- **Double Buffering**: Ping-pong buffers for position updates
- **Pinned Host Memory**: For fast CPU-GPU transfers when needed
- **Stream Synchronization**: Overlap computation and communication

#### Performance Optimization Checklist
1. **Kernel Optimization**
   - Maximize occupancy (target >75%)
   - Minimize divergent branches
   - Use fast math intrinsics (`rsqrtf`, `__fmul_rn`)
   - Exploit texture memory for read-only data

2. **Memory Optimization**
   - Coalesced global memory access
   - Effective shared memory utilization
   - Minimize CPU-GPU transfers
   - Use async memory operations

3. **Algorithm Optimization**
   - Spatial sorting for cache locality
   - Adaptive time stepping
   - Level-of-detail for distant particles
   - Early termination conditions

### Success Metrics

#### Performance Targets
- **1M particles**: 60 FPS sustained
- **100K particles**: 120+ FPS
- **10K particles**: 240+ FPS
- **GPU utilization**: >90% during computation
- **Memory bandwidth**: >80% of theoretical peak

#### Quality Metrics
- **Numerical accuracy**: Match CPU results within tolerance
- **Visual fidelity**: No noticeable artifacts
- **Stability**: No crashes or memory leaks
- **Compatibility**: Works across different GPU architectures

#### Development Metrics
- **Code maintainability**: Clean separation of CPU/GPU code
- **Testability**: Comprehensive unit and integration tests
- **Documentation**: Complete API and architecture documentation
- **Reproducibility**: Consistent results across runs

### Risk Mitigation

#### Technical Risks
- **CUDA compilation complexity**: Start with simple kernels, gradual complexity
- **Memory limitations**: Implement dynamic quality scaling
- **Platform compatibility**: Maintain CPU fallback throughout

#### Performance Risks
- **Memory bandwidth bottlenecks**: Optimize data layouts early
- **Kernel launch overhead**: Batch operations where possible
- **Synchronization costs**: Minimize CPU-GPU synchronization points

### Future Expansion Opportunities
- **Multi-GPU scaling**: Distribute computation across multiple cards
- **Ray tracing integration**: Use RT cores for advanced lighting
- **Machine learning**: GPU-based parameter optimization
- **Cloud deployment**: GPU instances for web-based simulation

## Phase 1.5: Hardware-Adaptive Particle Limits (Pre-GPU)

### Current Limitation
The O(n²) CPU algorithm has a fixed MAX_PARTICLES limit (currently 15K) that is not optimal for all hardware:
- **Mac M1**: 15K may be too high (limited performance cores)
- **Mac M3 Pro**: 15K works but can be sluggish at max
- **High-end workstations** (Xeon, Threadripper): Could handle 25-30K+
- **Current behavior**: UI hangs at max particles, reset can recover but difficult

### Proposed Solution: Hardware Detection & Dynamic Limits

#### 1. Server-Side Hardware Detection
Detect and categorize hardware on startup:
```rust
enum HardwareClass {
    LowEnd,      // < 8 cores, < 16GB RAM
    MidRange,    // 8-16 cores, 16-32GB RAM
    HighEnd,     // 16+ cores, 32GB+ RAM
    Workstation, // 24+ cores, 64GB+ RAM
}
```

#### 2. Particle Limit Table
Map hardware classes to safe particle limits:

| Hardware Class | CPU Cores | RAM | MAX_PARTICLES | Expected Frame Time |
|----------------|-----------|-----|---------------|---------------------|
| **Low-End** | 4-8 | 8-16GB | 8,000 | ~50ms |
| **Mid-Range** | 8-12 | 16-32GB | 15,000 | ~100ms |
| **High-End** | 12-24 | 32-64GB | 25,000 | ~150ms |
| **Workstation** | 24+ | 64GB+ | 35,000 | ~200ms |

**Apple Silicon Adjustments:**
- M1/M2: Treat as Mid-Range → 12K particles
- M3/M3 Pro: Mid-Range+ → 15K particles
- M3 Max/Ultra: High-End → 20-25K particles

#### 3. GPU-Based Limits (Post-CUDA Implementation)
Once GPU acceleration is available, create GPU-specific limits:

| GPU Type | VRAM | Compute | MAX_PARTICLES | Expected FPS |
|----------|------|---------|---------------|--------------|
| **Integrated** | Shared | Low | 10,000 | 30 FPS |
| **RTX 3060** | 12GB | Mid | 500,000 | 60 FPS |
| **RTX 4060/5060** | 12-16GB | Mid-High | 750,000 | 60 FPS |
| **P40/P100** | 16-24GB | High | 1,000,000 | 60 FPS |
| **A100/H100** | 40-80GB | Very High | 2,000,000+ | 60+ FPS |

#### 4. Deadman Switch / Auto-Recovery
Implement automatic recovery mechanisms:

**Option A: Computation Timeout**
```rust
// In simulation loop
if computation_time > TIMEOUT_MS {
    log::error!("Computation timeout - auto-reducing particles");
    self.config.particle_count = (self.config.particle_count as f32 * 0.7) as usize;
    self.reset();
}
```

**Option B: Server Restart on Hang**
- Watchdog detects hang (no frame progress for 30s)
- Send error message to client
- Gracefully restart simulation with reduced particle count
- Log incident for diagnostics

**Option C: Client-Side Circuit Breaker**
- Client detects no updates for 10 seconds
- Automatically reduces particle slider by 50%
- Sends reset command
- Notifies user: "Reduced particles due to performance"

#### 5. Implementation Priority
1. **Phase 1.5.1**: Add hardware detection and dynamic MAX_PARTICLES (1 week)
2. **Phase 1.5.2**: Implement auto-recovery / deadman switch (1 week)
3. **Phase 1.5.3**: Add GPU detection when CUDA ready (part of Phase 2)

#### 6. Configuration
Allow override in config.toml:
```toml
[simulation]
auto_detect_limits = true
force_max_particles = 15000  # Override auto-detection if needed
enable_auto_recovery = true
recovery_threshold_ms = 500  # Auto-reduce if frames take >500ms
```