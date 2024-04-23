use crate::model::AvailableCastle;
use crate::model::{Board, Piece, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION};
use crate::ui::UiError;

pub fn from_board(board: &Board) -> Result<String, UiError> {
    let pieces = pieces(board)?;
    let active_colour = match board.active_colour() {
        PieceColour::White => "w",
        PieceColour::Black => "b",
    };
    let castling_availability = castling_availability(board.available_castles());
    let en_passent_square = board
        .en_passant_square()
        .map_or("-".to_string(), ToString::to_string);

    Ok(format!(
        "{} {} {} {} {} {}",
        pieces,
        active_colour,
        castling_availability,
        en_passent_square,
        board.halfmove_clock(),
        board.fullmove_clock()
    ))
}

fn castling_availability(available_castles: &[AvailableCastle]) -> String {
    if available_castles.is_empty() {
        return "-".to_string();
    }
    available_castles
        .iter()
        .map(|castle| match castle {
            AvailableCastle::WhiteKingside => 'K',
            AvailableCastle::WhiteQueenside => 'Q',
            AvailableCastle::BlackKingside => 'k',
            AvailableCastle::BlackQueenside => 'q',
        })
        .collect()
}

fn pieces(board: &Board) -> Result<String, UiError> {
    let mut chars: Vec<char> = Vec::new();
    for row in (MIN_POSITION..=MAX_POSITION).rev() {
        let mut empty_columns = 0;
        for col in MIN_POSITION..=MAX_POSITION {
            let position = Position::new(row, col);
            if let Some(piece) = board.occupant(position) {
                if empty_columns > 0 {
                    chars.push(char::from_digit(empty_columns, 10).ok_or_else(|| {
                        UiError::new(
                            "Failed to parse char from empty columns in FEN string".to_string(),
                        )
                    })?);
                    empty_columns = 0;
                }
                chars.push(to_char(piece));
            } else {
                empty_columns += 1;
                if col == MAX_POSITION {
                    chars.push(char::from_digit(empty_columns, 10).ok_or_else(|| {
                        UiError::new(
                            "Failed to parse char from empty columns in FEN string".to_string(),
                        )
                    })?);
                }
            }
        }

        if row > MIN_POSITION {
            chars.push('/');
        }
    }
    Ok(chars.into_iter().collect())
}

fn to_char(piece: &Piece) -> char {
    match (piece.colour(), piece.piece_type()) {
        (PieceColour::Black, PieceType::Pawn) => 'p',
        (PieceColour::Black, PieceType::Knight) => 'n',
        (PieceColour::Black, PieceType::Bishop) => 'b',
        (PieceColour::Black, PieceType::Rook) => 'r',
        (PieceColour::Black, PieceType::Queen) => 'q',
        (PieceColour::Black, PieceType::King) => 'k',
        (PieceColour::White, PieceType::Pawn) => 'P',
        (PieceColour::White, PieceType::Knight) => 'N',
        (PieceColour::White, PieceType::Bishop) => 'B',
        (PieceColour::White, PieceType::Rook) => 'R',
        (PieceColour::White, PieceType::Queen) => 'Q',
        (PieceColour::White, PieceType::King) => 'K',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod castling_availability_tests {
        use super::*;

        #[test]
        fn returns_dash_if_no_castles_available() {
            assert_eq!(castling_availability(&[]), "-".to_string())
        }

        #[test]
        fn returns_formatted_castles() {
            let castles = &[
                AvailableCastle::WhiteQueenside,
                AvailableCastle::BlackKingside,
            ];
            assert_eq!(castling_availability(castles), "Qk".to_string())
        }
    }

    mod pieces_tests {
        use super::*;

        #[test]
        fn constucts_fen_string_from_board_positions() {
            let mut builder = Board::builder();
            builder.piece(
                Piece::new(PieceColour::Black, PieceType::Bishop),
                Position::new(7, 5),
            );
            builder.piece(
                Piece::new(PieceColour::White, PieceType::King),
                Position::new(3, 3),
            );

            let board = builder.build();
            assert_eq!(pieces(&board).unwrap(), "5b2/8/8/8/3K4/8/8/8".to_string())
        }
    }
}
