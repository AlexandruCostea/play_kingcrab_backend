use actix_web::{post, web, HttpResponse, Responder};
use std::sync::Mutex;

use crate::engine::engine::{Engine, EvaluatorType};
use crate::models::models::{SwitchBotRequest, BestMoveRequest, BestMoveResponse};


#[post("/switch_bot")]
pub async fn switch_bot(
    data: web::Data<Mutex<Engine>>,
    req: web::Json<SwitchBotRequest>,
) -> impl Responder {
    let mut engine = data.lock().unwrap();

    let strategy = match req.bot_type.to_lowercase().as_str() {
        "cnn" => EvaluatorType::CNN,
        "halfka" => EvaluatorType::HalfKA,
        _ => return HttpResponse::BadRequest().body("Unknown strategy"),
    };

    engine.switch_evaluator(strategy);
    HttpResponse::Ok().body("New bot initialized")
}


#[post("/get_best_move")]
pub async fn get_best_move(
    data: web::Data<Mutex<Engine>>,
    req: web::Json<BestMoveRequest>,
) -> impl Responder {
    let mut engine = data.lock().unwrap();
    let best = engine.search_best_move(&req.fen);
    match best {
        Ok(best_move) => HttpResponse::Ok().json(BestMoveResponse {best_move: best_move}),
        Err(e) => HttpResponse::BadRequest().body(format!("Error evaluating position: {}", e))
    }
}
