mod castle;
mod error;

use crate::model::{Board, MoveQualifier, Movement, PieceType, PlyMovement};

use self::error::EngineError;

pub fn execute_moves(board: &Board, ply: &[PlyMovement]) -> Result<Vec<Board>, EngineError> {
    todo!()
}

fn execute_move(board: &Board, ply: &PlyMovement) -> Result<Board, EngineError> {
    match ply {
        PlyMovement::KingsideCastle { colour, check: _ } => castle::kingside(board, *colour),
        PlyMovement::QueensideCastle { colour, check: _ } => castle::queenside(board, *colour),
        PlyMovement::Move {
            movement,
            qualifier,
            check: _,
        } => piece_move(board, movement, qualifier.as_ref(), None),
        PlyMovement::Promotion {
            movement,
            promotes_to,
            qualifier,
            check: _,
        } => piece_move(board, movement, qualifier.as_ref(), Some(promotes_to)),
    }
}

fn piece_move(
    board: &Board,
    movement: &Movement,
    qualifier: Option<&MoveQualifier>,
    promotes_to: Option<&PieceType>,
) -> Result<Board, EngineError> {
    todo!()
}
