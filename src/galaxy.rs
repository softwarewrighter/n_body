use nalgebra::{Point3, Vector3};
use crate::particle::Particle;

pub struct GalaxyGenerator;

impl GalaxyGenerator {
    pub fn new() -> Self {
        GalaxyGenerator
    }
    
    pub fn generate_spiral_galaxy(
        &self,
        num_particles: usize,
        center: Point3<f32>,
        bulk_velocity: Vector3<f32>,
        radius: f32,
        base_color: [f32; 4],
    ) -> Vec<Particle> {
        let mut particles = Vec::with_capacity(num_particles);
        
        for i in 0..num_particles {
            let t = i as f32 / num_particles as f32;
            
            // Spiral parameters
            let angle = t * std::f32::consts::PI * 4.0; // 2 full spirals
            let r = t * radius;
            
            // Add some randomness for thickness
            let thickness = 0.1 * radius;
            let rand_offset = self.pseudo_random(i);
            let z_offset = (rand_offset - 0.5) * thickness;
            
            // Position in galaxy frame
            let x = r * angle.cos();
            let y = r * angle.sin();
            let z = z_offset;
            
            let local_pos = Vector3::new(x, y, z);
            let position = center + local_pos;
            
            // Orbital velocity (simplified)
            let orbital_speed = (1.0 / (r + 0.1).sqrt()) * 2.0;
            let tangent = Vector3::new(-angle.sin(), angle.cos(), 0.0);
            let orbital_velocity = tangent * orbital_speed;
            
            let velocity = bulk_velocity + orbital_velocity;
            
            // Vary mass - more mass near center
            let mass = 1.0 + (1.0 - t) * 2.0;
            
            // Vary color slightly
            let color_variation = 0.2;
            let color = [
                base_color[0] + (rand_offset - 0.5) * color_variation,
                base_color[1] + (rand_offset - 0.5) * color_variation,
                base_color[2] + (rand_offset - 0.5) * color_variation,
                base_color[3],
            ];
            
            particles.push(Particle::new(position, velocity, mass, color));
        }
        
        particles
    }
    
    // Simple pseudo-random number generator for deterministic results
    fn pseudo_random(&self, seed: usize) -> f32 {
        let x = (seed.wrapping_mul(1103515245).wrapping_add(12345) >> 16) & 0x7fff;
        x as f32 / 32767.0
    }
}