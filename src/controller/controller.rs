use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Mutex;

use king_crab::{CNNEvaluator, HalfkaEvaluator};

use crate::engine::engine::Engine;
use crate::models::models::{BestMoveRequest, BestMoveResponse};



#[post("/get_best_move")]
pub async fn get_best_move(
    data: web::Data<Mutex<(Engine<CNNEvaluator>, Engine<HalfkaEvaluator>)>>,
    req: web::Json<BestMoveRequest>,
) -> impl Responder {
    let mut engines = data.lock().unwrap();

    let best = match req.engine_type.as_str().to_lowercase().as_str() {
        "cnn" => engines.0.search_best_move(&req.fen),
        "halfka" => engines.1.search_best_move(&req.fen),
        _ => return HttpResponse::BadRequest().body("Invalid engine type")
    };

    match best {
        Ok(best_move) => HttpResponse::Ok().json(BestMoveResponse {best_move: best_move}),
        Err(e) => HttpResponse::BadRequest().body(format!("Error evaluating position: {}", e))
    }
}