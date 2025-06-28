use nalgebra::Vector3;
use crate::particle::Particle;

pub struct PhysicsEngine {
    gravity_constant: f32,
    time_step: f32,
    softening: f32,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            gravity_constant: 1.0,
            time_step: 0.01,
            softening: 0.1, // Prevent singularities
        }
    }
    
    pub fn set_gravity_strength(&mut self, strength: f32) {
        self.gravity_constant = strength;
    }
    
    pub fn set_time_step(&mut self, dt: f32) {
        self.time_step = dt;
    }
    
    pub fn get_time_step(&self) -> f32 {
        self.time_step
    }
    
    pub fn update(&self, particles: &mut Vec<Particle>) {
        // Calculate accelerations for all particles
        let accelerations = self.calculate_accelerations(particles);
        
        // Update velocities and positions
        for (particle, acceleration) in particles.iter_mut().zip(accelerations.iter()) {
            particle.apply_acceleration(*acceleration, self.time_step);
            particle.update_position(self.time_step);
        }
    }
    
    fn calculate_accelerations(&self, particles: &[Particle]) -> Vec<Vector3<f32>> {
        let n = particles.len();
        let mut accelerations = vec![Vector3::zeros(); n];
        
        // O(n²) direct calculation - will optimize with Barnes-Hut later
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let diff = particles[j].position - particles[i].position;
                    let dist_sq = diff.magnitude_squared() + self.softening * self.softening;
                    let _dist = dist_sq.sqrt();
                    let force_magnitude = self.gravity_constant * particles[j].mass / dist_sq;
                    
                    accelerations[i] += diff.normalize() * force_magnitude;
                }
            }
        }
        
        accelerations
    }
}