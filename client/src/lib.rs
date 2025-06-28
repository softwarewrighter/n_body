use n_body_shared::{ClientMessage, ServerMessage, SimulationConfig, SimulationState};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, ErrorEvent, HtmlCanvasElement, MessageEvent, WebSocket};

mod renderer;
use renderer::Renderer;

#[wasm_bindgen]
pub struct Client {
    ws: WebSocket,
    renderer: Renderer,
    canvas: HtmlCanvasElement,
    current_state: Option<SimulationState>,
    config: SimulationConfig,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, server_url: String) -> Result<Client, JsValue> {
        console::log_1(&format!("Connecting to server: {}", server_url).into());
        
        let ws = WebSocket::new(&server_url)?;
        
        let renderer = Renderer::new(&canvas)?;
        
        let config = SimulationConfig {
            particle_count: 3000,
            time_step: 0.01,
            gravity_strength: 1.0,
            visual_fps: 30,
            zoom_level: 1.0,
        };
        
        Ok(Client {
            ws,
            renderer,
            canvas,
            current_state: None,
            config,
        })
    }
    
    pub fn start(&mut self) -> Result<(), JsValue> {
        self.resize();
        self.setup_websocket_handlers()?;
        Ok(())
    }
    
    fn setup_websocket_handlers(&self) -> Result<(), JsValue> {
        let ws = &self.ws;
        
        // On open
        let onopen = Closure::wrap(Box::new(move || {
            console::log_1(&"WebSocket connected".into());
            // Call global JavaScript function to update connection status
            let window = web_sys::window().unwrap();
            if let Some(handler) = window.get("updateConnectionStatus") {
                if let Some(function) = handler.dyn_ref::<js_sys::Function>() {
                    let _ = function.call1(&JsValue::NULL, &JsValue::from_bool(true));
                }
            }
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();
        
        // On message - this will be handled by JavaScript
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let message = String::from(txt);
                console::log_1(&format!("Received message: {}", message).into());
                
                // Call global JavaScript function to handle message
                let window = web_sys::window().unwrap();
                if let Some(handler) = window.get("handleWebSocketMessage") {
                    if let Some(function) = handler.dyn_ref::<js_sys::Function>() {
                        let _ = function.call1(&JsValue::NULL, &JsValue::from_str(&message));
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        
        // On error
        let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console::error_1(&format!("WebSocket error: {:?}", e).into());
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
        
        // On close
        let onclose = Closure::wrap(Box::new(move || {
            console::log_1(&"WebSocket closed".into());
            // Call global JavaScript function to update connection status
            let window = web_sys::window().unwrap();
            if let Some(handler) = window.get("updateConnectionStatus") {
                if let Some(function) = handler.dyn_ref::<js_sys::Function>() {
                    let _ = function.call1(&JsValue::NULL, &JsValue::from_bool(false));
                }
            }
        }) as Box<dyn FnMut()>);
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();
        
        Ok(())
    }
    
    pub fn handle_message(&mut self, message: String) {
        match serde_json::from_str::<ServerMessage>(&message) {
            Ok(msg) => match msg {
                ServerMessage::State(state) => {
                    console::log_1(&format!("Received {} particles", state.particles.len()).into());
                    self.current_state = Some(state);
                    self.render();
                }
                ServerMessage::Stats(stats) => {
                    // Stats are handled by JavaScript for UI updates
                    let stats_json = serde_json::to_string(&stats).unwrap();
                    web_sys::window()
                        .unwrap()
                        .get("updateStats")
                        .unwrap()
                        .dyn_ref::<js_sys::Function>()
                        .unwrap()
                        .call1(&JsValue::NULL, &JsValue::from_str(&stats_json))
                        .unwrap();
                }
                ServerMessage::Config(config) => {
                    console::log_1(&format!("Received config: {} particles", config.particle_count).into());
                    self.config = config.clone();
                    
                    // Update UI elements via JavaScript
                    let window = web_sys::window().unwrap();
                    if let Some(update_ui) = window.get("updateUIFromConfig") {
                        if let Some(function) = update_ui.dyn_ref::<js_sys::Function>() {
                            let config_json = serde_json::to_string(&config).unwrap();
                            let _ = function.call1(&JsValue::NULL, &JsValue::from_str(&config_json));
                        }
                    }
                }
            },
            Err(e) => {
                console::error_1(&format!("Failed to parse server message: {}", e).into());
            }
        }
    }
    
    fn render(&self) {
        if let Some(state) = &self.current_state {
            console::log_1(&format!("Rendering {} particles", state.particles.len()).into());
            self.renderer.render(&state.particles);
        }
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
        self.config.particle_count = count;
        if self.is_connected() {
            self.send_config_update();
        } else {
            console::log_1(&"Cannot update particle count: WebSocket not connected".into());
        }
    }
    
    pub fn set_time_step(&mut self, dt: f32) {
        self.config.time_step = dt;
        if self.is_connected() {
            self.send_config_update();
        } else {
            console::log_1(&"Cannot update time step: WebSocket not connected".into());
        }
    }
    
    pub fn set_gravity_strength(&mut self, strength: f32) {
        self.config.gravity_strength = strength;
        if self.is_connected() {
            self.send_config_update();
        } else {
            console::log_1(&"Cannot update gravity strength: WebSocket not connected".into());
        }
    }
    
    pub fn set_visual_fps(&mut self, fps: u32) {
        self.config.visual_fps = fps;
        if self.is_connected() {
            self.send_config_update();
        } else {
            console::log_1(&"Cannot update visual FPS: WebSocket not connected".into());
        }
    }
    
    pub fn set_zoom_level(&mut self, zoom: f32) {
        self.config.zoom_level = zoom;
        self.renderer.set_zoom(zoom);
        if self.is_connected() {
            self.send_config_update();
        } else {
            console::log_1(&"Cannot update zoom level: WebSocket not connected".into());
        }
    }
    
    fn is_connected(&self) -> bool {
        self.ws.ready_state() == WebSocket::OPEN
    }
    
    pub fn reset(&self) {
        if self.ws.ready_state() == WebSocket::OPEN {
            let msg = ClientMessage::Reset;
            if let Ok(json) = serde_json::to_string(&msg) {
                if let Err(e) = self.ws.send_with_str(&json) {
                    console::error_1(&format!("Failed to send reset: {:?}", e).into());
                }
            }
        } else {
            console::log_1(&"WebSocket not connected, cannot send reset".into());
        }
    }
    
    pub fn pause(&self) {
        if self.ws.ready_state() == WebSocket::OPEN {
            let msg = ClientMessage::Pause;
            if let Ok(json) = serde_json::to_string(&msg) {
                if let Err(e) = self.ws.send_with_str(&json) {
                    console::error_1(&format!("Failed to send pause: {:?}", e).into());
                }
            }
        }
    }
    
    pub fn resume(&self) {
        if self.ws.ready_state() == WebSocket::OPEN {
            let msg = ClientMessage::Resume;
            if let Ok(json) = serde_json::to_string(&msg) {
                if let Err(e) = self.ws.send_with_str(&json) {
                    console::error_1(&format!("Failed to send resume: {:?}", e).into());
                }
            }
        }
    }
    
    fn send_config_update(&self) {
        if self.ws.ready_state() == WebSocket::OPEN {
            let msg = ClientMessage::UpdateConfig(self.config.clone());
            if let Ok(json) = serde_json::to_string(&msg) {
                if let Err(e) = self.ws.send_with_str(&json) {
                    console::error_1(&format!("Failed to send config update: {:?}", e).into());
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"N-Body client WASM module loaded".into());
}