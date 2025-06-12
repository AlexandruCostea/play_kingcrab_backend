use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct BestMoveRequest {
    pub engine_type: String,
    pub fen: String
}

#[derive(Serialize)]
pub struct BestMoveResponse {
    pub best_move: String
}
