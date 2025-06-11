use king_crab::{self, engine::{board::fen::FenError}, Board, CNNEvaluator,
HalfkaEvaluator, MoveGenerator, Searcher, TranspositionTable};
pub enum EvaluatorType {
    CNN,
    HalfKA,
    None
}

pub struct Engine {
    cnn_depth: u8,
    halfka_depth: u8,

    board: Board,
    halfka_evaluator: HalfkaEvaluator,
    cnn_evaluator: CNNEvaluator,

    move_generator: MoveGenerator,
    transposition_table: TranspositionTable,

    evaluator_type: EvaluatorType
}


impl Engine {
    pub fn new(cnn_path: String, halfka_path: String, cnn_depth: u8, halfka_depth: u8) -> Self {
        let mut board = Board::new();
        board.from_fen(None).expect("Failed to initialize board");

        let halfka_evaluator = HalfkaEvaluator::new(&halfka_path).expect("Failed to load HalfKA");
        let cnn_evaluator = CNNEvaluator::new(&cnn_path).expect("Failed to load CNN");

        let move_generator = MoveGenerator::new();
        let transposition_table = TranspositionTable::new(20);

        Engine{
            cnn_depth,
            halfka_depth,
            board,
            halfka_evaluator,
            cnn_evaluator,
            move_generator,
            transposition_table,
            evaluator_type: EvaluatorType::None
        }
    }

    pub fn switch_evaluator(&mut self, new_evaluator_type: EvaluatorType) {
        self.evaluator_type = new_evaluator_type;
    }

    pub fn search_best_move(&mut self, fen_position: &str) -> Result<String, FenError> {
        self.board.from_fen(Some(fen_position))?;

        match self.evaluator_type {
            EvaluatorType::None => Ok("None".to_string()),
            EvaluatorType::CNN => {
                let mut searcher = Searcher::new(
                    &mut self.cnn_evaluator,
                    &self.move_generator,
                    &mut self.transposition_table
                );
                let mv = searcher.search(&self.board, self.cnn_depth)
                                        .unwrap();
                Ok(mv.to_string())
            },
            EvaluatorType::HalfKA => {
                let mut searcher = Searcher::new(
                    &mut self.halfka_evaluator,
                    &self.move_generator,
                    &mut self.transposition_table
                );
                let mv = searcher.search(&self.board, self.halfka_depth)
                                        .unwrap();
                Ok(mv.to_string())
            },
        }
    }
}