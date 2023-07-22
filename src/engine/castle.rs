use crate::model::{
    AvailableCastle, Board, Piece, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION,
};

use super::error::EngineError;
use lazy_static::lazy_static;

lazy_static! {
    static ref WHITE_KING_POSITION: Position = Position::new(MIN_POSITION, 4).unwrap();
    static ref WHITE_KINGS_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MAX_POSITION).unwrap();
    static ref WHITE_QUEENS_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MIN_POSITION).unwrap();
    static ref BLACK_KING_POSITION: Position = Position::new(MAX_POSITION, 4).unwrap();
    static ref BLACK_KINGS_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MAX_POSITION).unwrap();
    static ref BLACK_QUEENS_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MIN_POSITION).unwrap();
}

pub fn kingside(board: &Board, colour: PieceColour) -> Result<Board, EngineError> {
    let castle_type = match colour {
        PieceColour::White => AvailableCastle::WhiteKingside,
        PieceColour::Black => AvailableCastle::BlackKingside,
    };

    if !board.available_castles().contains(&castle_type) {
        return Err(EngineError::new(format!(
            "Kingside castle for {colour:?} is not a legal move"
        )));
    }

    let (king_position, rook_position) = match colour {
        PieceColour::White => (*WHITE_KING_POSITION, *WHITE_KINGS_ROOK_POSITION),
        PieceColour::Black => (*BLACK_KING_POSITION, *BLACK_KINGS_ROOK_POSITION),
    };

    let mut next_board = board.clone();
    next_board.add(Piece::new(colour, PieceType::King), rook_position);
    next_board.add(Piece::new(colour, PieceType::Rook), king_position);
    next_board.remove_available_castle(castle_type);

    Ok(next_board)
}

pub fn queenside(board: &Board, colour: PieceColour) -> Result<Board, EngineError> {
    let castle_type = match colour {
        PieceColour::White => AvailableCastle::WhiteQueenside,
        PieceColour::Black => AvailableCastle::BlackQueenside,
    };

    if !board.available_castles().contains(&castle_type) {
        return Err(EngineError::new(format!(
            "Queenside castle for {colour:?} is not a legal move"
        )));
    }

    let (king_position, rook_position) = match colour {
        PieceColour::White => (*WHITE_KING_POSITION, *WHITE_QUEENS_ROOK_POSITION),
        PieceColour::Black => (*BLACK_KING_POSITION, *BLACK_QUEENS_ROOK_POSITION),
    };

    let mut next_board = board.clone();
    next_board.add(Piece::new(colour, PieceType::King), rook_position);
    next_board.add(Piece::new(colour, PieceType::Rook), king_position);
    next_board.remove_available_castle(castle_type);

    Ok(next_board)
}

#[cfg(test)]
mod tests {
    use crate::model::BoardBuilder;

    use super::*;

    mod kingside_tests {
        use super::*;

        #[test]
        fn returns_err_if_castle_is_not_in_available_castles() {
            let board = board_with_available_castles(vec![AvailableCastle::WhiteKingside, AvailableCastle::BlackQueenside]);
            assert!(kingside(&board, PieceColour::Black).is_err())
        }

        #[test]
        fn performs_kingside_castle_for_white() {
            
        }
    }

    fn board_with_available_castles(available_castles: Vec<AvailableCastle>) -> Board {
        let mut board_builder = BoardBuilder::new();
        board_builder.available_castles(available_castles);
        board_builder.build()
    }
}