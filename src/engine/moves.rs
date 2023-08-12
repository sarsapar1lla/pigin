use crate::model::{Board, Piece, PieceColour, PieceType, Position, MAX_POSITION, MIN_POSITION};
use lazy_static::lazy_static;

const WHITE_PAWN_METRIC: (i8, i8) = (1, 0);
const WHITE_DOUBLE_PAWN_METRIC: (i8, i8) = (2, 0);
const BLACK_PAWN_METRIC: (i8, i8) = (-1, 0);
const BLACK_DOUBLE_PAWN_METRIC: (i8, i8) = (-2, 0);

const WHITE_PAWN_CAPTURES: &[(i8, i8)] = &[(1, -1), (1, 1)];
const BLACK_PAWN_CAPTURES: &[(i8, i8)] = &[(-1, -1), (-1, 1)];

const DIAGONAL_METRICS: &[(i8, i8)] = &[(1, -1), (1, 1), (-1, 1), (-1, -1)];
const LATERAL_METRICS: &[(i8, i8)] = &[(1, 0), (0, 1), (0, -1), (-1, 0)];
const KNIGHT_METRICS: &[(i8, i8)] = &[
    (1, -2),
    (2, -1),
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
];

lazy_static! {
    static ref ALL_METRICS: Vec<(i8, i8)> = [DIAGONAL_METRICS, LATERAL_METRICS].concat();
}

#[derive(Debug, PartialEq, Eq)]
enum MoveOutcome {
    Empty(Position),
    OccupiedOppositeColour(Position),
    OccupiedSameColour,
    Invalid,
}

pub fn find(piece: Piece, position: Position, board: &Board) -> Vec<Position> {
    let colour = *piece.colour();
    match piece.piece_type() {
        PieceType::Pawn => pawn_moves(position, colour, board),
        PieceType::Knight => apply_metrics_once(position, colour, KNIGHT_METRICS, board),
        PieceType::Bishop => apply_metrics(position, colour, DIAGONAL_METRICS, board),
        PieceType::Rook => apply_metrics(position, colour, LATERAL_METRICS, board),
        PieceType::Queen => apply_metrics(position, colour, &ALL_METRICS, board),
        PieceType::King => apply_metrics_once(position, colour, &ALL_METRICS, board),
    }
}

fn pawn_moves(position: Position, colour: PieceColour, board: &Board) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let move_metric = if colour == PieceColour::White {
        WHITE_PAWN_METRIC
    } else {
        BLACK_PAWN_METRIC
    };

    if let MoveOutcome::Empty(new_position) =
        apply_metric_once(position, colour, move_metric, board)
    {
        positions.push(new_position);
    }

    let on_home_row = (colour == PieceColour::White && position.row() == MIN_POSITION + 1)
        | (colour == PieceColour::Black && position.row() == MAX_POSITION - 1);

    if on_home_row && !positions.is_empty() {
        let double_move_metric = if colour == PieceColour::White {
            WHITE_DOUBLE_PAWN_METRIC
        } else {
            BLACK_DOUBLE_PAWN_METRIC
        };
        if let MoveOutcome::Empty(new_position) =
            apply_metric_once(position, colour, double_move_metric, board)
        {
            positions.push(new_position);
        }
    }

    let capture_metrics = if colour == PieceColour::White {
        WHITE_PAWN_CAPTURES
    } else {
        BLACK_PAWN_CAPTURES
    };
    let mut capture_moves: Vec<Position> = capture_metrics
        .iter()
        .filter_map(
            |&metric| match apply_metric_once(position, colour, metric, board) {
                MoveOutcome::OccupiedOppositeColour(new_position) => Some(new_position),
                MoveOutcome::Empty(new_position) if Some(&new_position) == board.en_passant_square() => Some(new_position),
                _ => None,
            },
        )
        .collect();

    positions.append(&mut capture_moves);

    positions
}

fn apply_metrics(
    position: Position,
    colour: PieceColour,
    metrics: &[(i8, i8)],
    board: &Board,
) -> Vec<Position> {
    metrics
        .iter()
        .flat_map(|&metric| apply_metric(position, colour, metric, board))
        .collect()
}

fn apply_metric(
    mut position: Position,
    colour: PieceColour,
    metric: (i8, i8),
    board: &Board,
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    loop {
        match apply_metric_once(position, colour, metric, board) {
            MoveOutcome::Invalid | MoveOutcome::OccupiedSameColour => return positions,
            MoveOutcome::OccupiedOppositeColour(new_position) => {
                positions.push(new_position);
                return positions;
            }
            MoveOutcome::Empty(new_position) => {
                positions.push(new_position);
                position = new_position
            }
        }
    }
}

fn apply_metrics_once(
    position: Position,
    colour: PieceColour,
    metrics: &[(i8, i8)],
    board: &Board,
) -> Vec<Position> {
    metrics
        .iter()
        .filter_map(
            |&metric| match apply_metric_once(position, colour, metric, board) {
                MoveOutcome::Empty(position) => Some(position),
                MoveOutcome::OccupiedOppositeColour(position) => Some(position),
                MoveOutcome::OccupiedSameColour => None,
                MoveOutcome::Invalid => None,
            },
        )
        .collect()
}

fn apply_metric_once(
    position: Position,
    colour: PieceColour,
    metric: (i8, i8),
    board: &Board,
) -> MoveOutcome {
    let row = position.row() + metric.0;
    let col = position.col() + metric.1;
    let new_position = Position::try_from(row, col);

    new_position
        .map(|p| match board.occupant(p) {
            None => MoveOutcome::Empty(p),
            Some(piece) if piece.colour() != &colour => MoveOutcome::OccupiedOppositeColour(p),
            _ => MoveOutcome::OccupiedSameColour,
        })
        .unwrap_or_else(|_| MoveOutcome::Invalid)
}

#[cfg(test)]
mod tests {
    use crate::model::BoardBuilder;

    use super::*;

    mod white_pawn_moves_tests {
        use super::*;

        #[test]
        fn blocks_forward_move_if_occupied() {
            let positions = pawn_moves(
                Position::new(1, 3),
                PieceColour::White,
                &board(),
            );
            assert!(positions.is_empty())
        }

        #[test]
        fn finds_pawn_move() {
            let positions = pawn_moves(
                Position::new(2, 5),
                PieceColour::White,
                &board(),
            );
            assert_eq!(positions, vec![Position::new(3, 5)])
        }

        #[test]
        fn finds_double_move_if_on_home_row() {
            let positions = pawn_moves(
                Position::new(1, 5),
                PieceColour::White,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(2, 5),
                    Position::new(3, 5),
                ]
            )
        }

        #[test]
        fn finds_capture_if_available() {
            let positions = pawn_moves(
                Position::new(1, 4),
                PieceColour::White,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(2, 4),
                    Position::new(3, 4),
                    Position::new(2, 3)
                ]
            )
        }

        #[test]
        fn finds_en_passant_capture_if_available() {
            let positions = pawn_moves(
                Position::new(4, 4),
                PieceColour::White,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(5, 4),
                    Position::new(5, 5),
                ]
            )
        }

        fn board() -> Board {
            let mut builder = BoardBuilder::new();
            builder.piece(
                Piece::new(PieceColour::Black, PieceType::Rook),
                Position::new(2, 3),
            );
            builder.en_passant_square(Position::new(5, 5));
            builder.build()
        }
    }

    mod black_pawn_moves_tests {
        use super::*;

        #[test]
        fn blocks_forward_move_if_occupied() {
            let positions = pawn_moves(
                Position::new(6, 3),
                PieceColour::Black,
                &board(),
            );
            assert!(positions.is_empty())
        }

        #[test]
        fn finds_pawn_move() {
            let positions = pawn_moves(
                Position::new(5, 5),
                PieceColour::Black,
                &board(),
            );
            assert_eq!(positions, vec![Position::new(4, 5)])
        }

        #[test]
        fn finds_double_move_if_on_home_row() {
            let positions = pawn_moves(
                Position::new(6, 5),
                PieceColour::Black,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(5, 5),
                    Position::new(4, 5)
                ]
            )
        }

        #[test]
        fn finds_capture_if_available() {
            let positions = pawn_moves(
                Position::new(6, 4),
                PieceColour::Black,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(5, 4),
                    Position::new(4, 4),
                    Position::new(5, 3)
                ]
            )
        }

        #[test]
        fn finds_en_passant_capture_if_available() {
            let positions = pawn_moves(
                Position::new(5, 6),
                PieceColour::Black,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::new(4, 6),
                    Position::new(4, 7)
                ]
            )
        }

        fn board() -> Board {
            let mut builder = BoardBuilder::new();
            builder.piece(
                Piece::new(PieceColour::White, PieceType::Rook),
                Position::new(5, 3),
            );
            builder.en_passant_square(Position::new(4, 7));
            builder.build()
        }
    }

    mod apply_metrics_tests {
        use super::*;

        #[test]
        fn applies_list_of_metrics_until_blocked() {
            let metrics = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
            let positions = apply_metrics(
                Position::try_from(0, 3).unwrap(),
                PieceColour::White,
                metrics,
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::try_from(1, 3).unwrap(),
                    Position::try_from(2, 3).unwrap(),
                    Position::try_from(0, 2).unwrap()
                ]
            )
        }

        fn board() -> Board {
            let mut builder = BoardBuilder::new();
            builder.piece(
                Piece::new(PieceColour::Black, PieceType::Rook),
                Position::try_from(2, 3).unwrap(),
            );
            builder.piece(
                Piece::new(PieceColour::Black, PieceType::Knight),
                Position::try_from(0, 2).unwrap(),
            );
            builder.piece(
                Piece::new(PieceColour::White, PieceType::Rook),
                Position::try_from(0, 4).unwrap(),
            );
            builder.build()
        }
    }

    mod apply_metric_tests {
        use super::*;

        #[test]
        fn returns_once_new_position_invalid() {
            let positions = apply_metric(
                Position::try_from(1, 3).unwrap(),
                PieceColour::White,
                (-1, 0),
                &board(),
            );
            assert_eq!(positions, vec![Position::try_from(0, 3).unwrap()])
        }

        #[test]
        fn returns_once_new_position_occupied_by_same_colour() {
            let positions = apply_metric(
                Position::try_from(0, 3).unwrap(),
                PieceColour::Black,
                (1, 0),
                &board(),
            );
            assert_eq!(positions, vec![Position::try_from(1, 3).unwrap()])
        }

        #[test]
        fn returns_once_new_position_occupied_by_opposite_colour_including_new_position() {
            let positions = apply_metric(
                Position::try_from(0, 3).unwrap(),
                PieceColour::White,
                (1, 0),
                &board(),
            );
            assert_eq!(
                positions,
                vec![
                    Position::try_from(1, 3).unwrap(),
                    Position::try_from(2, 3).unwrap()
                ]
            )
        }

        fn board() -> Board {
            let mut builder = BoardBuilder::new();
            builder.piece(
                Piece::new(PieceColour::Black, PieceType::Rook),
                Position::try_from(2, 3).unwrap(),
            );
            builder.build()
        }
    }

    mod apply_metrics_once_tests {
        use super::*;

        #[test]
        fn applies_list_of_metrics_once_each() {
            let metrics = &[(-1, 0), (1, 0), (0, 1)];
            let positions = apply_metrics_once(
                Position::try_from(0, 0).unwrap(),
                PieceColour::Black,
                metrics,
                &board(),
            );

            assert_eq!(positions, vec![Position::try_from(0, 1).unwrap()])
        }
    }

    mod apply_metric_once_tests {
        use super::*;

        #[test]
        fn returns_none_if_new_position_is_invalid() {
            let position = apply_metric_once(
                Position::try_from(0, 0).unwrap(),
                PieceColour::White,
                (-1, 0),
                &board(),
            );
            assert_eq!(position, MoveOutcome::Invalid)
        }

        #[test]
        fn returns_none_if_new_position_is_occupied_by_same_piece_colour() {
            let position = apply_metric_once(
                Position::try_from(0, 0).unwrap(),
                PieceColour::Black,
                (1, 0),
                &board(),
            );
            assert_eq!(position, MoveOutcome::OccupiedSameColour)
        }

        #[test]
        fn returns_new_position_if_occupied_by_opposite_piece_colour() {
            let position = apply_metric_once(
                Position::try_from(0, 0).unwrap(),
                PieceColour::White,
                (1, 0),
                &board(),
            );
            assert_eq!(
                position,
                MoveOutcome::OccupiedOppositeColour(Position::try_from(1, 0).unwrap())
            )
        }

        #[test]
        fn returns_new_position_if_unoccupied() {
            let position = apply_metric_once(
                Position::try_from(0, 0).unwrap(),
                PieceColour::White,
                (0, 1),
                &board(),
            );
            assert_eq!(
                position,
                MoveOutcome::Empty(Position::try_from(0, 1).unwrap())
            )
        }
    }

    fn board() -> Board {
        let mut builder = BoardBuilder::new();
        builder.piece(
            Piece::new(PieceColour::Black, PieceType::Rook),
            Position::try_from(1, 0).unwrap(),
        );
        builder.build()
    }
}
