mod castle;
mod en_passant;
mod error;
mod legality;
mod moves;

use crate::model::{
    AvailableCastle, Board, MoveQualifier, Movement, Piece, PieceColour, PieceType, Ply,
    PlyMovement, Position,
};

use self::{
    castle::{
        BLACK_KINGS_ROOK_POSITION, BLACK_QUEENS_ROOK_POSITION, WHITE_KINGS_ROOK_POSITION,
        WHITE_QUEENS_ROOK_POSITION,
    },
    error::EngineError,
};

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

// TODO: write tests
fn piece_move(
    board: &Board,
    movement: &Movement,
    qualifier: Option<&MoveQualifier>,
    promotes_to: Option<&PieceType>,
) -> Result<Board, EngineError> {
    let piece = movement.piece();
    let position = movement.position();

    let candidates = board.search(piece);
    let king_position = *board
        .search(Piece::new(*piece.colour(), PieceType::King))
        .first()
        .ok_or_else(|| {
            EngineError::new(format!(
                "Could not locate king of colour {:?}",
                piece.colour()
            ))
        })?;
    let opposition_colour = match *piece.colour() {
        PieceColour::White => PieceColour::Black,
        PieceColour::Black => PieceColour::White,
    };

    if candidates.is_empty() {
        return Err(EngineError::new(format!(
            "No candidates found for piece {piece:?}"
        )));
    }

    let viable_candidates: Vec<Position> = candidates
        .into_iter()
        .filter(|&candidate_position| {
            moves::find(piece, candidate_position, board).contains(&position)
                && legality::check(
                    piece,
                    candidate_position,
                    position,
                    king_position,
                    opposition_colour,
                    board.clone(),
                )
        })
        .collect();

    let candidate = match &viable_candidates[..] {
        [] => Err(EngineError::new(format!(
            "No piece {piece:?} can move to position {position:?}"
        ))),
        [candidate] => Ok(*candidate),
        candidates => {
            if let Some(qualifier) = qualifier {
                qualified_position(candidates, qualifier)
            } else {
                Err(EngineError::new(format!(
                    "Cannot determine candidate {piece:?} moving to {position:?} given {} candidates: no qualifier provided",
                    candidates.len()
                )))
            }
        }
    }?;

    let mut next_board = board.clone();
    next_board.remove(candidate);

    if let Some(&en_passant_square) = board.en_passant_square() {
        en_passant::current(piece, position, en_passant_square, &mut next_board);
    }

    match promotes_to {
        None => next_board.add(piece, position),
        Some(&other) => next_board.add(Piece::new(*piece.colour(), other), position),
    };

    en_passant::next(piece, candidate, position, &mut next_board);
    update_available_castles(piece, candidate, &mut next_board);

    Ok(next_board)
}

// TODO: write tests
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

// TODO: consider removing this
fn update_available_castles(piece: Piece, position: Position, board: &mut Board) -> &mut Board {
    match (piece.piece_type(), piece.colour()) {
        (PieceType::King, PieceColour::White) => {
            board.remove_available_castle(AvailableCastle::WhiteKingside);
            board.remove_available_castle(AvailableCastle::WhiteQueenside);
        }
        (PieceType::King, PieceColour::Black) => {
            board.remove_available_castle(AvailableCastle::BlackKingside);
            board.remove_available_castle(AvailableCastle::BlackQueenside);
        }
        (PieceType::Rook, PieceColour::White) => {
            if position == *WHITE_KINGS_ROOK_POSITION {
                board.remove_available_castle(AvailableCastle::WhiteKingside);
            } else if position == *WHITE_QUEENS_ROOK_POSITION {
                board.remove_available_castle(AvailableCastle::WhiteQueenside);
            }
        }
        (PieceType::Rook, PieceColour::Black) => {
            if position == *BLACK_KINGS_ROOK_POSITION {
                board.remove_available_castle(AvailableCastle::BlackKingside);
            } else if position == *BLACK_QUEENS_ROOK_POSITION {
                board.remove_available_castle(AvailableCastle::BlackQueenside);
            }
        }
        _ => {}
    };
    board
}
