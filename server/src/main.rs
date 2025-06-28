use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;
use std::sync::{Arc, Mutex};

mod physics;
mod simulation;
mod websocket;

use simulation::Simulation;
use websocket::SimulationWebSocket;

pub struct AppState {
    simulation: Arc<Mutex<Simulation>>,
}

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let simulation = data.simulation.clone();
    ws::start(SimulationWebSocket::new(simulation), &req, stream)
}

async fn index() -> Result<HttpResponse, Error> {
    info!("Index route called");
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let num_threads = num_cpus::get();
    info!("Starting N-Body server with {} CPU threads", num_threads);
    
    // Initialize rayon with all available threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let simulation = Arc::new(Mutex::new(Simulation::new()));
    let app_state = web::Data::new(AppState { simulation });

    info!("Server starting at http://localhost:8080");
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
            .service(actix_files::Files::new("/pkg", "server/pkg").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}