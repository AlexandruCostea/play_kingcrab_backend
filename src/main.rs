use actix_web::{web, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenvy::dotenv;

use std::sync::Mutex;
use std::env;

pub mod engine;
pub mod models;
pub mod controller;

use crate::engine::engine::Engine;
use crate::controller::controller::{get_best_move, switch_bot};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let cnn_path: String = env::var("CNN_MODEL_PATH").expect("CNN_PATH must be set in .env file");
    let cnn_depth: u8 = env::var("CNN_DEPTH").expect("CNN_DEPTH must be set in .env file")
        .parse()
        .expect("CNN_DEPTH must be a valid number");

    let halfka_path: String = env::var("HALFKA_MODEL_PATH").expect("HALFKA_MODEL_PATH must be set in .env file");
    let halfka_depth: u8 = env::var("HALFKA_DEPTH").expect("HALFKA_DEPTH must be set in .env file")
        .parse()
        .expect("HALFKA_DEPTH must be a valid number");

    let engine = Engine::new(cnn_path, halfka_path, cnn_depth, halfka_depth);
    let shared_engine = web::Data::new(Mutex::new(engine));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173") 
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(shared_engine.clone())
            .service(switch_bot)
            .service(get_best_move)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
