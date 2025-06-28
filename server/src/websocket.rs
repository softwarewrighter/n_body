use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use log::{error, info};
use n_body_shared::{ClientMessage, ServerMessage};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::simulation::Simulation;

use crate::config::{SimulationConfig, WebSocketConfig};

pub struct SimulationWebSocket {
    simulation: Arc<Mutex<Simulation>>,
    last_heartbeat: Instant,
    last_render: Instant,
    last_physics_update: Instant,
    ws_config: WebSocketConfig,
    sim_config: SimulationConfig,
}

impl SimulationWebSocket {
    pub fn new(
        simulation: Arc<Mutex<Simulation>>,
        ws_config: &WebSocketConfig,
        sim_config: &SimulationConfig,
    ) -> Self {
        Self {
            simulation,
            last_heartbeat: Instant::now(),
            last_render: Instant::now(),
            last_physics_update: Instant::now(),
            ws_config: ws_config.clone(),
            sim_config: sim_config.clone(),
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
        // Run at configured update rate
        let update_interval = Duration::from_millis(self.sim_config.update_rate_ms);

        ctx.run_interval(update_interval, |act, ctx| {
            // Step physics simulation
            if act.last_physics_update.elapsed()
                >= Duration::from_millis(act.sim_config.update_rate_ms)
            {
                act.last_physics_update = Instant::now();

                // Check if context is still valid (client connected)
                if ctx.state() != actix::ActorState::Running {
                    return;
                }

                let (state, stats) = {
                    match act.simulation.lock() {
                        Ok(mut sim) => sim.step(),
                        Err(e) => {
                            error!("Failed to lock simulation: {}", e);
                            return;
                        }
                    }
                };

                // Check current visual FPS setting
                let visual_fps = {
                    match act.simulation.lock() {
                        Ok(sim) => sim.get_config().visual_fps,
                        Err(_) => 30, // fallback
                    }
                };
                let render_interval_ms = 1000 / visual_fps;

                // Only send state update if enough time has passed for visual FPS
                if act.last_render.elapsed().as_millis() >= render_interval_ms as u128 {
                    act.last_render = Instant::now();

                    // Send state update with error handling
                    match serde_json::to_string(&ServerMessage::State(state)) {
                        Ok(json) => ctx.text(json),
                        Err(e) => error!("Failed to serialize state: {}", e),
                    }
                }

                // Send stats every 30 frames
                if stats.frame_number % 30 == 0 {
                    match serde_json::to_string(&ServerMessage::Stats(stats)) {
                        Ok(json) => ctx.text(json),
                        Err(e) => error!("Failed to serialize stats: {}", e),
                    }
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

        // Send initial config with error handling
        match self.simulation.lock() {
            Ok(sim) => {
                let config = sim.get_config().clone();
                match serde_json::to_string(&ServerMessage::Config(config)) {
                    Ok(json) => ctx.text(json),
                    Err(e) => error!("Failed to serialize initial config: {}", e),
                }
            }
            Err(e) => {
                error!("Failed to lock simulation for initial config: {}", e);
                // Close connection if we can't access simulation
                ctx.stop();
            }
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
                self.last_heartbeat = Instant::now();

                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(msg) => {
                        match self.simulation.lock() {
                            Ok(mut sim) => {
                                match msg {
                                    ClientMessage::UpdateConfig(config) => {
                                        info!("Updating config: {:?}", config);
                                        sim.update_config(config);

                                        // Send back updated config to confirm
                                        let updated_config = sim.get_config().clone();
                                        if let Ok(json) = serde_json::to_string(
                                            &ServerMessage::Config(updated_config),
                                        ) {
                                            ctx.text(json);
                                        }
                                    }
                                    ClientMessage::Reset => {
                                        info!("Resetting simulation");
                                        sim.reset();

                                        // Send immediate state update after reset
                                        let (state, _) = sim.step();
                                        if let Ok(json) =
                                            serde_json::to_string(&ServerMessage::State(state))
                                        {
                                            ctx.text(json);
                                        }
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
                            Err(e) => {
                                error!("Failed to lock simulation: {}", e);
                                // Send error message back to client
                                if let Ok(json) =
                                    serde_json::to_string(&"Server error: simulation lock failed")
                                {
                                    ctx.text(json);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse client message '{}': {}", text, e);
                        // Send error message back to client
                        if let Ok(json) = serde_json::to_string(&format!("Parse error: {}", e)) {
                            ctx.text(json);
                        }
                    }
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
