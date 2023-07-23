mod castle;
mod error;

use crate::model::{
    Board, MoveQualifier, Movement, PieceColour, PieceType, Ply, PlyMovement, Position,
};

use self::error::EngineError;

pub fn execute_moves(board: &Board, ply: &[Ply]) -> Result<Vec<Board>, EngineError> {
    let mut boards: Vec<Board> = vec![board.clone()];

    let mut current_board = board;
    for ply in ply {
        let next_board = execute_move(current_board, ply.movement())?;
        boards.push(next_board);
        current_board = boards
            .last()
            .ok_or_else(|| EngineError::new("Failed to retrieve previously generated board"))?;
    }
    Ok(boards)
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
    let piece = movement.piece();
    let position = movement.position();

    let candidates = board.search(piece);

    if candidates.is_empty() {
        return Err(EngineError::new(format!(
            "No candidates found for piece {piece:?}"
        )));
    }

    let candidate_position = match &candidates[..] {
        [] => Err(EngineError::new(format!(
            "No candidates found for piece {piece:?}"
        ))),
        [candidate] => Ok(candidate.clone()),
        candidates => {
            if let Some(qualifier) = qualifier {
                qualified_position(candidates, qualifier)
            } else {
                Err(EngineError::new(format!(
                    "Cannot determine piece position from {}: no qualifier provided",
                    candidates.len()
                )))
            }
        }
    }?;

    

    todo!()
}

fn qualified_position(
    candidates: &[Position],
    qualifier: &MoveQualifier,
) -> Result<Position, EngineError> {
    match qualifier {
        MoveQualifier::Position(position) => Ok(*position),
        MoveQualifier::Col(col) => {
            let filtered_candidates: Vec<Position> = candidates
                .iter()
                .filter_map(|&position| {
                    if position.col() == *col {
                        Some(position)
                    } else {
                        None
                    }
                })
                .collect();
            match filtered_candidates[..] {
                [only] => Ok(only),
                _ => Err(EngineError::new(format!("Cannot uniquely determine piece position from qualifier {qualifier:?}; candidates: {candidates:?}")))
            }
        }
        MoveQualifier::Row(row) => {
            let filtered_candidates: Vec<Position> = candidates
                .iter()
                .filter_map(|&position| {
                    if position.row() == *row {
                        Some(position)
                    } else {
                        None
                    }
                })
                .collect();
            match filtered_candidates[..] {
                [only] => Ok(only),
                _ => Err(EngineError::new(format!("Cannot uniquely determine piece position from qualifier {qualifier:?}; candidates: {candidates:?}")))
            }
        }
    }
}
