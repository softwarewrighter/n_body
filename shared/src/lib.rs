use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Particle {
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub mass: f32,
    pub color: [f32; 4],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationState {
    pub particles: Vec<Particle>,
    pub sim_time: f32,
    pub frame_number: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SimulationConfig {
    pub particle_count: usize,
    pub time_step: f32,
    pub gravity_strength: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationStats {
    pub fps: f32,
    pub computation_time_ms: f32,
    pub particle_count: usize,
    pub sim_time: f32,
    pub cpu_usage: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    UpdateConfig(SimulationConfig),
    Reset,
    Pause,
    Resume,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    State(SimulationState),
    Stats(SimulationStats),
    Config(SimulationConfig),
}