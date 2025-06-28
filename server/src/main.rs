use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;
use std::sync::{Arc, Mutex};

mod config;
mod physics;
mod simulation;
mod websocket;

use config::Config;
use simulation::Simulation;
use websocket::SimulationWebSocket;

pub struct AppState {
    simulation: Arc<Mutex<Simulation>>,
    config: Config,
}

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let simulation = data.simulation.clone();
    let ws_config = &data.config.websocket;
    ws::start(SimulationWebSocket::new(simulation, ws_config), &req, stream)
}

async fn index() -> Result<HttpResponse, Error> {
    info!("Index route called");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../www/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::load();
    
    let num_threads = num_cpus::get();
    info!("Starting N-Body server with {} CPU threads", num_threads);
    
    // Initialize rayon with all available threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let simulation = Arc::new(Mutex::new(Simulation::new(&config.simulation)));
    let app_state = web::Data::new(AppState { 
        simulation,
        config: config.clone(),
    });

    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    info!("Server starting at http://{}:{}", config.server.host, config.server.port);
    info!("Current working directory: {:?}", std::env::current_dir());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(ws_index))
            .service(actix_files::Files::new("/", "www").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}