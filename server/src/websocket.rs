use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use log::{error, info};
use n_body_shared::{ClientMessage, ServerMessage};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::simulation::Simulation;

use crate::config::WebSocketConfig;

pub struct SimulationWebSocket {
    simulation: Arc<Mutex<Simulation>>,
    last_heartbeat: Instant,
    ws_config: WebSocketConfig,
}

impl SimulationWebSocket {
    pub fn new(simulation: Arc<Mutex<Simulation>>, ws_config: &WebSocketConfig) -> Self {
        Self {
            simulation,
            last_heartbeat: Instant::now(),
            ws_config: ws_config.clone(),
        }
    }
    
    fn start_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        let heartbeat_interval = Duration::from_secs(self.ws_config.heartbeat_interval_sec);
        let client_timeout = Duration::from_secs(self.ws_config.client_timeout_sec);
        
        ctx.run_interval(heartbeat_interval, move |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > client_timeout {
                info!("WebSocket client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
    
    fn start_simulation_loop(&self, ctx: &mut <Self as Actor>::Context) {
        let update_interval = Duration::from_millis(33); // Use default for now, could be configurable
        
        ctx.run_interval(update_interval, |act, ctx| {
            let (state, stats) = {
                let mut sim = act.simulation.lock().unwrap();
                sim.step()
            };
            
            // Send state update
            if let Ok(json) = serde_json::to_string(&ServerMessage::State(state)) {
                ctx.text(json);
            }
            
            // Send stats every 30 frames
            if stats.sim_time as u64 % 30 == 0 {
                if let Ok(json) = serde_json::to_string(&ServerMessage::Stats(stats)) {
                    ctx.text(json);
                }
            }
        });
    }
}

impl Actor for SimulationWebSocket {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection established");
        self.start_heartbeat(ctx);
        self.start_simulation_loop(ctx);
        
        // Send initial config
        let config = self.simulation.lock().unwrap().get_config().clone();
        if let Ok(json) = serde_json::to_string(&ServerMessage::Config(config)) {
            ctx.text(json);
        }
    }
    
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WebSocket connection closed");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SimulationWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        let mut sim = self.simulation.lock().unwrap();
                        match msg {
                            ClientMessage::UpdateConfig(config) => {
                                info!("Updating config: {:?}", config);
                                sim.update_config(config);
                            }
                            ClientMessage::Reset => {
                                info!("Resetting simulation");
                                sim.reset();
                            }
                            ClientMessage::Pause => {
                                info!("Pausing simulation");
                                sim.set_paused(true);
                            }
                            ClientMessage::Resume => {
                                info!("Resuming simulation");
                                sim.set_paused(false);
                            }
                        }
                    }
                    Err(e) => error!("Failed to parse client message: {}", e),
                }
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                ctx.stop();
            }
            _ => {}
        }
    }
}