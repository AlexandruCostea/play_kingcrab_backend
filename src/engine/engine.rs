use king_crab::{engine::board::fen::FenError, Board,
    Evaluator, MoveGenerator, Searcher, TranspositionTable};


pub struct Engine<T: Evaluator> {
    board: Board,

    depth: u8,
    evaluator: T,
    move_generator: MoveGenerator,
    transposition_table: TranspositionTable,
}


impl<T> Engine<T> where T: Evaluator {
    pub fn new(evaluator: T, depth: u8) -> Self {
        let mut board = Board::new();
        board.from_fen(None).expect("Failed to initialize board");

        let move_generator = MoveGenerator::new();
        let transposition_table = TranspositionTable::new(20);

        Engine{
            board,
            depth,
            evaluator,
            move_generator,
            transposition_table,
        }
    }


    pub fn search_best_move(&mut self, fen_position: &str) -> Result<String, FenError> {
        self.board.from_fen(Some(fen_position))?;

        let mut searcher = Searcher::new(
            &mut self.evaluator,
            &self.move_generator,
            &mut self.transposition_table
        );

        let mv = searcher.search(&self.board, self.depth)
                                    .map(|m| m.to_string());

        match mv {
            Some(mv_str) => Ok(mv_str),
            None => Err(FenError::PieceSquarePartError(
                "No valid move found".to_string()
            ))
        }
    }
}