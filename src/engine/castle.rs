use crate::model::{
    AvailableCastle, Board, Piece, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION,
};

use super::error::EngineError;
use lazy_static::lazy_static;

lazy_static! {
    static ref WHITE_KING_POSITION: Position = Position::new(MIN_POSITION, 4).unwrap();
    static ref WHITE_KINGSIDE_CASTLE_KING_POSITION: Position =
        Position::new(MIN_POSITION, MAX_POSITION - 1).unwrap();
    static ref WHITE_QUEENSIDE_CASTLE_KING_POSITION: Position =
        Position::new(MIN_POSITION, MIN_POSITION + 2).unwrap();
    pub static ref WHITE_KINGS_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MAX_POSITION).unwrap();
    static ref WHITE_KINGSIDE_CASTLE_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MAX_POSITION - 2).unwrap();
    pub static ref WHITE_QUEENS_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MIN_POSITION).unwrap();
    static ref WHITE_QUEENSIDE_CASTLE_ROOK_POSITION: Position =
        Position::new(MIN_POSITION, MIN_POSITION + 3).unwrap();
    static ref BLACK_KING_POSITION: Position = Position::new(MAX_POSITION, 4).unwrap();
    static ref BLACK_KINGSIDE_CASTLE_KING_POSITION: Position =
        Position::new(MAX_POSITION, MAX_POSITION - 1).unwrap();
    static ref BLACK_QUEENSIDE_CASTLE_KING_POSITION: Position =
        Position::new(MAX_POSITION, MIN_POSITION + 2).unwrap();
    pub static ref BLACK_KINGS_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MAX_POSITION).unwrap();
    static ref BLACK_KINGSIDE_CASTLE_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MAX_POSITION - 2).unwrap();
    pub static ref BLACK_QUEENS_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MIN_POSITION).unwrap();
    static ref BLACK_QUEENSIDE_CASTLE_ROOK_POSITION: Position =
        Position::new(MAX_POSITION, MIN_POSITION + 3).unwrap();
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

    let (king_position_before, rook_position_before, king_position_after, rook_position_after) =
        match colour {
            PieceColour::White => (
                *WHITE_KING_POSITION,
                *WHITE_KINGS_ROOK_POSITION,
                *WHITE_KINGSIDE_CASTLE_KING_POSITION,
                *WHITE_KINGSIDE_CASTLE_ROOK_POSITION,
            ),
            PieceColour::Black => (
                *BLACK_KING_POSITION,
                *BLACK_KINGS_ROOK_POSITION,
                *BLACK_KINGSIDE_CASTLE_KING_POSITION,
                *BLACK_KINGSIDE_CASTLE_ROOK_POSITION,
            ),
        };

    let mut next_board = board.clone();
    next_board.remove(king_position_before);
    next_board.remove(rook_position_before);
    next_board.add(Piece::new(colour, PieceType::King), king_position_after);
    next_board.add(Piece::new(colour, PieceType::Rook), rook_position_after);

    remove_castling_for_colour(&mut next_board, colour);

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

    let (king_position_before, rook_position_before, king_position_after, rook_position_after) =
        match colour {
            PieceColour::White => (
                *WHITE_KING_POSITION,
                *WHITE_QUEENS_ROOK_POSITION,
                *WHITE_QUEENSIDE_CASTLE_KING_POSITION,
                *WHITE_QUEENSIDE_CASTLE_ROOK_POSITION,
            ),
            PieceColour::Black => (
                *BLACK_KING_POSITION,
                *BLACK_QUEENS_ROOK_POSITION,
                *BLACK_QUEENSIDE_CASTLE_KING_POSITION,
                *BLACK_QUEENSIDE_CASTLE_ROOK_POSITION,
            ),
        };

    let mut next_board = board.clone();
    next_board.remove(king_position_before);
    next_board.remove(rook_position_before);
    next_board.add(Piece::new(colour, PieceType::King), king_position_after);
    next_board.add(Piece::new(colour, PieceType::Rook), rook_position_after);

    remove_castling_for_colour(&mut next_board, colour);

    Ok(next_board)
}

fn remove_castling_for_colour(board: &mut Board, colour: PieceColour) -> &mut Board {
    match colour {
        PieceColour::White => {
            board.remove_available_castle(AvailableCastle::WhiteKingside);
            board.remove_available_castle(AvailableCastle::WhiteQueenside);
            board
        }
        PieceColour::Black => {
            board.remove_available_castle(AvailableCastle::BlackKingside);
            board.remove_available_castle(AvailableCastle::BlackQueenside);
            board
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::BoardBuilder;

    use super::*;

    mod kingside_tests {
        use super::*;

        #[test]
        fn returns_err_if_castle_is_not_in_available_castles() {
            let mut board_builder = BoardBuilder::new();
            board_builder.available_castles(vec![
                AvailableCastle::WhiteKingside,
                AvailableCastle::BlackQueenside,
            ]);
            let board = board_builder.build();

            assert!(kingside(&board, PieceColour::Black).is_err())
        }

        #[test]
        fn performs_kingside_castle_for_white() {
            let mut board_builder = BoardBuilder::new();
            board_builder
                .available_castles(vec![
                    AvailableCastle::WhiteKingside,
                    AvailableCastle::WhiteQueenside,
                    AvailableCastle::BlackKingside,
                ])
                .piece(
                    Piece::new(PieceColour::White, PieceType::King),
                    *WHITE_KING_POSITION,
                )
                .piece(
                    Piece::new(PieceColour::White, PieceType::Rook),
                    *WHITE_KINGS_ROOK_POSITION,
                );

            let board = board_builder.build();
            let new_board = kingside(&board, PieceColour::White).unwrap();

            assert!(new_board.occupant(*WHITE_KING_POSITION).is_none());
            assert!(new_board.occupant(*WHITE_KINGS_ROOK_POSITION).is_none());
            assert_eq!(
                new_board.available_castles(),
                vec![AvailableCastle::BlackKingside]
            );
            assert_eq!(
                new_board.occupant(*WHITE_KINGSIDE_CASTLE_KING_POSITION),
                Some(&Piece::new(PieceColour::White, PieceType::King))
            );
            assert_eq!(
                new_board.occupant(*WHITE_KINGSIDE_CASTLE_ROOK_POSITION),
                Some(&Piece::new(PieceColour::White, PieceType::Rook))
            );
        }

        #[test]
        fn performs_kingside_castle_for_black() {
            let mut board_builder = BoardBuilder::new();
            board_builder
                .available_castles(vec![
                    AvailableCastle::BlackKingside,
                    AvailableCastle::BlackQueenside,
                    AvailableCastle::WhiteKingside,
                ])
                .piece(
                    Piece::new(PieceColour::Black, PieceType::King),
                    *BLACK_KING_POSITION,
                )
                .piece(
                    Piece::new(PieceColour::Black, PieceType::Rook),
                    *BLACK_KINGS_ROOK_POSITION,
                );

            let board = board_builder.build();
            let new_board = kingside(&board, PieceColour::Black).unwrap();

            assert!(new_board.occupant(*BLACK_KING_POSITION).is_none());
            assert!(new_board.occupant(*BLACK_KINGS_ROOK_POSITION).is_none());
            assert_eq!(
                new_board.available_castles(),
                vec![AvailableCastle::WhiteKingside]
            );
            assert_eq!(
                new_board.occupant(*BLACK_KINGSIDE_CASTLE_KING_POSITION),
                Some(&Piece::new(PieceColour::Black, PieceType::King))
            );
            assert_eq!(
                new_board.occupant(*BLACK_KINGSIDE_CASTLE_ROOK_POSITION),
                Some(&Piece::new(PieceColour::Black, PieceType::Rook))
            );
        }
    }

    mod queenside_tests {
        use super::*;

        #[test]
        fn returns_err_if_castle_is_not_in_available_castles() {
            let mut board_builder = BoardBuilder::new();
            board_builder.available_castles(vec![
                AvailableCastle::WhiteQueenside,
                AvailableCastle::BlackKingside,
            ]);
            let board = board_builder.build();

            assert!(queenside(&board, PieceColour::Black).is_err())
        }

        #[test]
        fn performs_queenside_castle_for_white() {
            let mut board_builder = BoardBuilder::new();
            board_builder
                .available_castles(vec![
                    AvailableCastle::WhiteKingside,
                    AvailableCastle::WhiteQueenside,
                    AvailableCastle::BlackKingside,
                ])
                .piece(
                    Piece::new(PieceColour::White, PieceType::King),
                    *WHITE_KING_POSITION,
                )
                .piece(
                    Piece::new(PieceColour::White, PieceType::Rook),
                    *WHITE_QUEENS_ROOK_POSITION,
                );

            let board = board_builder.build();
            let new_board = queenside(&board, PieceColour::White).unwrap();

            assert!(new_board.occupant(*WHITE_KING_POSITION).is_none());
            assert!(new_board.occupant(*WHITE_KINGS_ROOK_POSITION).is_none());
            assert_eq!(
                new_board.available_castles(),
                vec![AvailableCastle::BlackKingside]
            );
            assert_eq!(
                new_board.occupant(*WHITE_QUEENSIDE_CASTLE_KING_POSITION),
                Some(&Piece::new(PieceColour::White, PieceType::King))
            );
            assert_eq!(
                new_board.occupant(*WHITE_QUEENSIDE_CASTLE_ROOK_POSITION),
                Some(&Piece::new(PieceColour::White, PieceType::Rook))
            );
        }

        #[test]
        fn performs_kingside_castle_for_black() {
            let mut board_builder = BoardBuilder::new();
            board_builder
                .available_castles(vec![
                    AvailableCastle::BlackKingside,
                    AvailableCastle::BlackQueenside,
                    AvailableCastle::WhiteKingside,
                ])
                .piece(
                    Piece::new(PieceColour::Black, PieceType::King),
                    *BLACK_KING_POSITION,
                )
                .piece(
                    Piece::new(PieceColour::Black, PieceType::Rook),
                    *BLACK_QUEENS_ROOK_POSITION,
                );

            let board = board_builder.build();
            let new_board = queenside(&board, PieceColour::Black).unwrap();

            assert!(new_board.occupant(*BLACK_KING_POSITION).is_none());
            assert!(new_board.occupant(*BLACK_KINGS_ROOK_POSITION).is_none());
            assert_eq!(
                new_board.available_castles(),
                vec![AvailableCastle::WhiteKingside]
            );
            assert_eq!(
                new_board.occupant(*BLACK_QUEENSIDE_CASTLE_KING_POSITION),
                Some(&Piece::new(PieceColour::Black, PieceType::King))
            );
            assert_eq!(
                new_board.occupant(*BLACK_QUEENSIDE_CASTLE_ROOK_POSITION),
                Some(&Piece::new(PieceColour::Black, PieceType::Rook))
            );
        }
    }

    mod remove_castling_for_colour_tests {
        use super::*;

        #[test]
        fn removes_castling_for_colour() {
            let mut board_builder = BoardBuilder::new();
            board_builder.available_castles(vec![
                AvailableCastle::WhiteKingside,
                AvailableCastle::WhiteQueenside,
                AvailableCastle::BlackKingside,
            ]);
            let mut board = board_builder.build();

            remove_castling_for_colour(&mut board, PieceColour::White);
            assert_eq!(
                board.available_castles(),
                vec![AvailableCastle::BlackKingside]
            )
        }
    }
}
