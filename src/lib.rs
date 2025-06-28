use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement};

mod particle;
mod physics;
mod renderer;
mod galaxy;

use particle::Particle;
use physics::PhysicsEngine;
use renderer::Renderer;
use galaxy::GalaxyGenerator;

#[wasm_bindgen]
pub struct Simulation {
    particles: Vec<Particle>,
    physics: PhysicsEngine,
    renderer: Renderer,
    canvas: HtmlCanvasElement,
    is_paused: bool,
    frame_count: u32,
    last_fps_update: f64,
    fps: f32,
    sim_time: f32,
    frame_time: f32,
    particle_count: usize,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SimulationStats {
    pub fps: f32,
    pub sim_time: f32,
    pub particle_count: usize,
    pub frame_time: f32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Simulation, JsValue> {
        console::log_1(&"Initializing N-Body Simulation".into());
        
        let renderer = Renderer::new(&canvas)?;
        let physics = PhysicsEngine::new();
        
        let mut sim = Simulation {
            particles: Vec::new(),
            physics,
            renderer,
            canvas,
            is_paused: false,
            frame_count: 0,
            last_fps_update: 0.0,
            fps: 0.0,
            sim_time: 0.0,
            frame_time: 0.0,
            particle_count: 10000,
        };
        
        sim.reset();
        Ok(sim)
    }
    
    pub fn start(&self) {
        console::log_1(&"Starting simulation".into());
    }
    
    pub fn reset(&mut self) {
        console::log_1(&format!("Resetting with {} particles", self.particle_count).into());
        
        // Generate two spiral galaxies
        let galaxy_gen = GalaxyGenerator::new();
        
        // First galaxy at (-5, 0, 0) moving right
        let galaxy1 = galaxy_gen.generate_spiral_galaxy(
            self.particle_count / 2,
            Point3::new(-5.0, 0.0, 0.0),
            Vector3::new(0.5, 0.0, 0.0),
            2.0, // radius
            [0.8, 0.8, 1.0, 1.0], // blueish
        );
        
        // Second galaxy at (5, 0, 0) moving left
        let galaxy2 = galaxy_gen.generate_spiral_galaxy(
            self.particle_count / 2,
            Point3::new(5.0, 0.0, 0.0),
            Vector3::new(-0.5, 0.0, 0.0),
            2.0, // radius
            [1.0, 0.8, 0.8, 1.0], // reddish
        );
        
        self.particles = [galaxy1, galaxy2].concat();
        self.sim_time = 0.0;
    }
    
    pub fn resize(&mut self) {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        
        self.renderer.resize(width, height);
    }
    
    pub fn set_particle_count(&mut self, count: usize) {
        self.particle_count = count;
        self.reset();
    }
    
    pub fn set_time_step(&mut self, dt: f32) {
        self.physics.set_time_step(dt);
    }
    
    pub fn set_gravity_strength(&mut self, strength: f32) {
        self.physics.set_gravity_strength(strength);
    }
    
    pub fn toggle_pause(&mut self) -> bool {
        self.is_paused = !self.is_paused;
        self.is_paused
    }
    
    pub fn get_stats(&self) -> SimulationStats {
        SimulationStats {
            fps: self.fps,
            sim_time: self.sim_time,
            particle_count: self.particles.len(),
            frame_time: self.frame_time,
        }
    }
    
    pub fn step(&mut self) {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        let start_time = performance.now();
        
        if !self.is_paused {
            // Update physics
            self.physics.update(&mut self.particles);
            self.sim_time += self.physics.get_time_step();
        }
        
        // Render
        self.renderer.render(&self.particles);
        
        // Update stats
        self.frame_count += 1;
        let current_time = performance.now();
        self.frame_time = (current_time - start_time) as f32;
        
        if current_time - self.last_fps_update >= 1000.0 {
            self.fps = (self.frame_count as f32) / ((current_time - self.last_fps_update) as f32 / 1000.0);
            self.frame_count = 0;
            self.last_fps_update = current_time;
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"N-Body WASM module loaded".into());
}