use actix_web::{web, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::sync::Mutex;
use std::env;

use king_crab::{CNNEvaluator, HalfkaEvaluator};

use play_kingcrab_backend::Engine;
use play_kingcrab_backend::get_best_move;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL")
                        .expect("FRONTEND_URL must be set in .env file");

    let(cnn_engine, halfka_engine) = init_engines();
    let shared_engines = web::Data::new(Mutex::new(
        (cnn_engine, halfka_engine)
    ));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(shared_engines.clone())
            .service(get_best_move)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


fn init_engines() -> (Engine<CNNEvaluator>, Engine<HalfkaEvaluator>) {
    let cnn_path: String = env::var("CNN_MODEL_PATH")
                            .expect("CNN_PATH must be set in .env file");
    let cnn_depth: u8 = env::var("CNN_DEPTH")
                            .expect("CNN_DEPTH must be set in .env file")
        .parse()
        .expect("CNN_DEPTH must be a valid number");

    let halfka_path: String = env::var("HALFKA_MODEL_PATH")
                                .expect("HALFKA_MODEL_PATH must be set in .env file");
    let halfka_depth: u8 = env::var("HALFKA_DEPTH")
                                .expect("HALFKA_DEPTH must be set in .env file")
        .parse()
        .expect("HALFKA_DEPTH must be a valid number");

    let cnn = CNNEvaluator::new(&cnn_path)
        .expect("Failed to load CNN model");

    let halfka = HalfkaEvaluator::new(&halfka_path)
        .expect("Failed to load HalfKA model");


    let cnn_engine = Engine::new(cnn, cnn_depth);
    let halfka_engine = Engine::new(halfka, halfka_depth);

    (cnn_engine, halfka_engine)
}