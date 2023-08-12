use crate::model::{Piece, Position, PieceType, Board, PieceColour, MIN_POSITION, MAX_POSITION};

pub fn next(piece: Piece, from: Position, to: Position, board: &mut Board) -> &mut Board {
    if piece.piece_type() != &PieceType::Pawn {
        return board;
    }

    let from_home_row = (piece.colour() == &PieceColour::White && from.row() == MIN_POSITION + 1)
    | (piece.colour() == &PieceColour::Black && from.row() == MAX_POSITION - 1);

    let rows_moved = (to.row() - from.row()).abs();

    if from_home_row && rows_moved == 2 {
        let en_passant_square = match *piece.colour() {
            PieceColour::White => Position::new(MIN_POSITION + 2, to.col()),
            PieceColour::Black => Position::new(MAX_POSITION - 2, to.col())
        };
        board.update_en_passant_square(en_passant_square);
    }

    board
}

pub fn current(piece: Piece, position: Position, en_passant_square: Position, board: &mut Board) -> &mut Board {
    if position == en_passant_square && piece.piece_type() == &PieceType::Pawn {
        match *piece.colour() {
            PieceColour::White => board.remove(Position::new(position.row() - 1, position.col())),
            PieceColour::Black => board.remove(Position::new(position.row() + 1, position.col()))
        }
    }

    board.remove_en_passant_square();
    board
}

#[cfg(test)]
mod tests {
    use super::*;

    mod next_tests {
        // TODO: write tests
    }

    mod current_tests {
        use crate::model::BoardBuilder;

        use super::*;

        #[test]
        fn removes_en_passant_square() {
            let mut board = board();
            current(Piece::new(PieceColour::White, PieceType::Bishop), Position::new(2, 2), Position::new(3, 5), &mut board);
            assert!(board.en_passant_square().is_none())
        }

        #[test]
        fn does_not_remove_pawn_if_piece_moves_to_en_passant_square() {
            let mut board = board();
            current(Piece::new(PieceColour::White, PieceType::Bishop), Position::new(6, 6), Position::new(6, 6), &mut board);
            assert_eq!(board.occupant(Position::new(5, 6)), Some(&Piece::new(PieceColour::Black, PieceType::Pawn)))
        }

        #[test]
        fn removes_black_pawn_if_pawn_moves_to_en_passant_square() {
            let mut board = board();
            current(Piece::new(PieceColour::White, PieceType::Pawn), Position::new(6, 6), Position::new(6, 6), &mut board);
            assert!(board.occupant(Position::new(5, 6)).is_none())
        }

        #[test]
        fn removes_white_pawn_if_pawn_moves_to_en_passant_square() {
            let mut board = board();
            current(Piece::new(PieceColour::Black, PieceType::Pawn), Position::new(2, 2), Position::new(2, 2), &mut board);
            assert!(board.occupant(Position::new(3, 2)).is_none())
        }

        fn board() -> Board {
            let mut builder = BoardBuilder::new();
            builder.piece(Piece::new(PieceColour::White, PieceType::Pawn), Position::new(3, 2));
            builder.piece(Piece::new(PieceColour::Black, PieceType::Pawn), Position::new(5, 6));
            builder.en_passant_square(Position::new(3, 5));
            builder.build()
        }
    }
}
