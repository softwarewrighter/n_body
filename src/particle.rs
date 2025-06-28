use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Particle {
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub mass: f32,
    pub color: [f32; 4], // RGBA
}

impl Particle {
    pub fn new(position: Point3<f32>, velocity: Vector3<f32>, mass: f32, color: [f32; 4]) -> Self {
        Particle {
            position,
            velocity,
            mass,
            color,
        }
    }
    
    pub fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
    
    pub fn apply_acceleration(&mut self, acceleration: Vector3<f32>, dt: f32) {
        self.velocity += acceleration * dt;
    }
}