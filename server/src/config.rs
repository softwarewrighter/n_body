use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub simulation: SimulationConfig,
    pub websocket: WebSocketConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimulationConfig {
    pub default_particles: usize,
    pub update_rate_ms: u64,
    pub stats_frequency: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebSocketConfig {
    pub heartbeat_interval_sec: u64,
    pub client_timeout_sec: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                port: 4000,
                host: "0.0.0.0".to_string(),
                debug: false,
            },
            simulation: SimulationConfig {
                default_particles: 1000,
                update_rate_ms: 33, // ~30 FPS
                stats_frequency: 30,
            },
            websocket: WebSocketConfig {
                heartbeat_interval_sec: 5,
                client_timeout_sec: 10,
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";
        
        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(mut config) => {
                        log::info!("Loaded configuration from {}", config_path);
                        
                        // Check for debug environment variable override
                        if std::env::var("N_BODY_DEBUG").is_ok() {
                            config.server.debug = true;
                            log::info!("Debug mode enabled via N_BODY_DEBUG environment variable");
                        }
                        
                        config
                    }
                    Err(e) => {
                        log::warn!("Failed to parse {}: {}. Using defaults.", config_path, e);
                        Self::default()
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read {}: {}. Using defaults.", config_path, e);
                    Self::default()
                }
            }
        } else {
            log::info!("No config.toml found, using default configuration");
            let mut config = Self::default();
            
            // Check for debug environment variable override
            if std::env::var("N_BODY_DEBUG").is_ok() {
                config.server.debug = true;
                log::info!("Debug mode enabled via N_BODY_DEBUG environment variable");
            }
            
            // Write default config file
            if let Ok(toml_str) = toml::to_string_pretty(&config) {
                if let Err(e) = fs::write(config_path, toml_str) {
                    log::warn!("Failed to write default config: {}", e);
                }
            }
            
            config
        }
    }
}