use super::moves;
use crate::model::{Board, Piece, PieceColour, PieceType, Position};

const RANGED_PIECES: &[PieceType] = &[PieceType::Bishop, PieceType::Rook, PieceType::Queen];

pub fn check(
    piece: Piece,
    from: Position,
    to: Position,
    king_position: Position,
    colour: PieceColour,
    mut board: Board,
) -> bool {
    if *piece.piece_type() == PieceType::King {
        return true;
    }

    board.remove(from);
    board.add(piece, to);

    !RANGED_PIECES
        .iter()
        .map(|&piece_type| can_capture_king(king_position, Piece::new(colour, piece_type), &board))
        .any(|x| x)
}

fn can_capture_king(king_position: Position, piece: Piece, board: &Board) -> bool {
    let pieces = board.search(piece);

    pieces
        .into_iter()
        .map(|position| moves::find(piece, position, board))
        .any(|positions| positions.contains(&king_position))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::BoardBuilder;

    mod check_tests {
        use super::*;

        #[test]
        fn returns_true_if_piece_is_king() {
            let check = check(
                Piece::new(PieceColour::Black, PieceType::King),
                Position::new(4, 4),
                Position::new(5, 3),
                Position::new(6, 5),
                PieceColour::White,
                board(),
            );
            assert!(check)
        }

        #[test]
        fn returns_true_if_no_ranged_piece_can_capture_king_after_move() {
            let check = check(
                Piece::new(PieceColour::Black, PieceType::Bishop),
                Position::new(4, 4),
                Position::new(5, 3),
                Position::new(6, 5),
                PieceColour::White,
                board(),
            );
            assert!(check)
        }

        #[test]
        fn returns_true_if_piece_still_blocks_ranged_pieces_after_move() {
            let check = check(
                Piece::new(PieceColour::Black, PieceType::Bishop),
                Position::new(4, 4),
                Position::new(5, 5),
                Position::new(6, 5),
                PieceColour::White,
                board(),
            );
            assert!(check)
        }

        #[test]
        fn returns_false_if_any_ranged_piece_can_capture_king_after_move() {
            let check = check(
                Piece::new(PieceColour::Black, PieceType::Bishop),
                Position::new(4, 4),
                Position::new(5, 3),
                Position::new(6, 6),
                PieceColour::White,
                board(),
            );
            assert!(!check)
        }
    }

    mod can_capture_king_tests {
        use super::*;

        #[test]
        fn returns_false_if_no_pieces_of_type_on_board() {
            let check = can_capture_king(
                Position::new(6, 4),
                Piece::new(PieceColour::White, PieceType::Rook),
                &board(),
            );
            assert!(!check)
        }

        #[test]
        fn returns_false_if_no_piece_of_type_can_capture_king() {
            let check = can_capture_king(
                Position::new(6, 4),
                Piece::new(PieceColour::White, PieceType::Bishop),
                &board(),
            );
            assert!(!check)
        }

        #[test]
        fn returns_true_if_piece_of_type_can_capture_king() {
            let check = can_capture_king(
                Position::new(6, 6),
                Piece::new(PieceColour::White, PieceType::Bishop),
                &board(),
            );
            assert!(check)
        }
    }

    fn board() -> Board {
        let mut builder = BoardBuilder::new();
        builder.piece(
            Piece::new(PieceColour::White, PieceType::Bishop),
            Position::new(3, 3),
        );
        builder.build()
    }
}
