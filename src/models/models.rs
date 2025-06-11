use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SwitchBotRequest {
    pub bot_type: String
}

#[derive(Deserialize)]
pub struct BestMoveRequest {
    pub fen: String
}

#[derive(Serialize)]
pub struct BestMoveResponse {
    pub best_move: String
}
