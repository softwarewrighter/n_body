use n_body_shared::{Particle, SimulationConfig, SimulationState, SimulationStats};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;
use std::time::Instant;

pub struct Simulation {
    particles: Vec<Particle>,
    config: SimulationConfig,
    sim_time: f32,
    frame_number: u64,
    is_paused: bool,
    last_computation_time: f32,
}

impl Simulation {
    pub fn new(sim_config: &crate::config::SimulationConfig) -> Self {
        let config = SimulationConfig {
            particle_count: sim_config.default_particles,
            time_step: 0.01,
            gravity_strength: 1.0,
        };
        
        let mut sim = Simulation {
            particles: Vec::new(),
            config,
            sim_time: 0.0,
            frame_number: 0,
            is_paused: false,
            last_computation_time: 0.0,
        };
        
        sim.reset();
        sim
    }
    
    pub fn reset(&mut self) {
        self.particles = generate_galaxy_collision(self.config.particle_count);
        self.sim_time = 0.0;
        self.frame_number = 0;
    }
    
    pub fn update_config(&mut self, config: SimulationConfig) {
        let need_reset = self.config.particle_count != config.particle_count;
        let old_count = self.config.particle_count;
        let new_count = config.particle_count;
        self.config = config;
        
        if need_reset {
            // Log the particle count change for better UX feedback
            log::info!("Particle count changed from {} to {}, resetting simulation", old_count, new_count);
            self.reset();
        }
    }
    
    pub fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }
    
    pub fn step(&mut self) -> (SimulationState, SimulationStats) {
        let start = Instant::now();
        
        if !self.is_paused {
            // Parallel physics computation using rayon
            let accelerations = self.calculate_accelerations_parallel();
            
            // Update particles in parallel
            self.particles
                .par_iter_mut()
                .zip(accelerations.par_iter())
                .for_each(|(particle, &acceleration)| {
                    particle.velocity += acceleration * self.config.time_step;
                    particle.position += particle.velocity * self.config.time_step;
                });
            
            self.sim_time += self.config.time_step;
            self.frame_number += 1;
        }
        
        self.last_computation_time = start.elapsed().as_secs_f32() * 1000.0;
        
        let state = SimulationState {
            particles: self.particles.clone(),
            sim_time: self.sim_time,
            frame_number: self.frame_number,
        };
        
        let stats = SimulationStats {
            fps: if self.last_computation_time > 0.0 {
                1000.0 / self.last_computation_time
            } else {
                0.0
            },
            computation_time_ms: self.last_computation_time,
            particle_count: self.particles.len(),
            sim_time: self.sim_time,
            cpu_usage: self.estimate_cpu_usage(),
            frame_number: self.frame_number,
        };
        
        (state, stats)
    }
    
    fn calculate_accelerations_parallel(&self) -> Vec<Vector3<f32>> {
        let n = self.particles.len();
        let softening = 0.1f32;
        let gravity = self.config.gravity_strength;
        
        // Use rayon to parallelize the outer loop
        (0..n)
            .into_par_iter()
            .map(|i| {
                let mut acceleration = Vector3::zeros();
                let particle_i = &self.particles[i];
                
                // Inner loop remains sequential but is parallelized across different i values
                for j in 0..n {
                    if i != j {
                        let particle_j = &self.particles[j];
                        let diff = particle_j.position - particle_i.position;
                        let dist_sq = diff.magnitude_squared() + softening * softening;
                        let force_magnitude = gravity * particle_j.mass / dist_sq;
                        
                        acceleration += diff.normalize() * force_magnitude;
                    }
                }
                
                acceleration
            })
            .collect()
    }
    
    fn estimate_cpu_usage(&self) -> f32 {
        // Rough estimate based on computation time and expected frame time
        let target_frame_time = 16.67; // 60 FPS target
        (self.last_computation_time / target_frame_time * 100.0).min(100.0)
    }
    
    pub fn get_config(&self) -> &SimulationConfig {
        &self.config
    }
}

fn generate_galaxy_collision(total_particles: usize) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(total_particles);
    
    // First galaxy
    particles.extend(generate_spiral_galaxy(
        total_particles / 2,
        Point3::new(-5.0, 0.0, 0.0),
        Vector3::new(0.5, 0.0, 0.0),
        2.0,
        [0.8, 0.8, 1.0, 1.0], // Blue
    ));
    
    // Second galaxy
    particles.extend(generate_spiral_galaxy(
        total_particles / 2,
        Point3::new(5.0, 0.0, 0.0),
        Vector3::new(-0.5, 0.0, 0.0),
        2.0,
        [1.0, 0.8, 0.8, 1.0], // Red
    ));
    
    particles
}

fn generate_spiral_galaxy(
    num_particles: usize,
    center: Point3<f32>,
    bulk_velocity: Vector3<f32>,
    radius: f32,
    base_color: [f32; 4],
) -> Vec<Particle> {
    (0..num_particles)
        .map(|i| {
            let t = i as f32 / num_particles as f32;
            let angle = t * std::f32::consts::PI * 4.0;
            let r = t * radius;
            
            let thickness = 0.1 * radius;
            let z_offset = (pseudo_random(i) - 0.5) * thickness;
            
            let x = r * angle.cos();
            let y = r * angle.sin();
            let z = z_offset;
            
            let local_pos = Vector3::new(x, y, z);
            let position = center + local_pos;
            
            let orbital_speed = (1.0 / (r + 0.1).sqrt()) * 2.0;
            let tangent = Vector3::new(-angle.sin(), angle.cos(), 0.0);
            let orbital_velocity = tangent * orbital_speed;
            
            let velocity = bulk_velocity + orbital_velocity;
            let mass = 1.0 + (1.0 - t) * 2.0;
            
            let color_variation = 0.2;
            let rand = pseudo_random(i);
            let color = [
                base_color[0] + (rand - 0.5) * color_variation,
                base_color[1] + (rand - 0.5) * color_variation,
                base_color[2] + (rand - 0.5) * color_variation,
                base_color[3],
            ];
            
            Particle {
                position,
                velocity,
                mass,
                color,
            }
        })
        .collect()
}

fn pseudo_random(seed: usize) -> f32 {
    let x = (seed.wrapping_mul(1103515245).wrapping_add(12345) >> 16) & 0x7fff;
    x as f32 / 32767.0
}