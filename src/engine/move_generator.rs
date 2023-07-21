// use crate::engine::transformer::Transformer;
use crate::model::{Piece, PieceColour, PieceType, Position, MAX_POSITION};

// static WHITE_FORWARD_TRANSFORMER: &[Transformer] = &[(1, 0)];
// static BLACK_FORWARD_TRANSFORMER: &[Transformer] = &[(-1, 0)];

// static SIDEWAYS_TRANFORMERS: &[Transformer] = &[(0, 1), (0, -1)];
// static DIAGONAL_TRANSFORMERS: &[Transformer] = &[(-1, -1), (-1, 1), (1, -1), (1, 1)];
// static KNIGHT_TRANSFORMERS: &[Transformer] = &[(2, -1), (2, 1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

// struct MoveGenerator {
//     max_movement: i8,
//     transformers: Vec<Box<dyn Transformer>>,
// }

// impl MoveGenerator {
// pub fn for_piece(piece: &Piece) -> Self {
//     let colour = piece.colour();
//     let piece_type = piece.piece_type();

//     let linear_transformers = {
//         let mut transformers = Vec::from(WHITE_FORWARD_TRANSFORMER);
//         transformers.extend_from_slice(BLACK_FORWARD_TRANSFORMER);
//         transformers.extend_from_slice(SIDEWAYS_TRANFORMERS);
//         transformers
//     };

//     let all_transformers = {
//         let mut transformers = linear_transformers.clone();
//         transformers.extend_from_slice(DIAGONAL_TRANSFORMERS);
//         transformers
//     };

//     match piece_type {
//         PieceType::Pawn => {
//             if *colour == PieceColour::White {
//                 MoveGenerator::new(1, WHITE_FORWARD_TRANSFORMER.into())
//             } else {
//                 MoveGenerator::new(1, BLACK_FORWARD_TRANSFORMER.into())
//             }
//         },
//         PieceType::Knight => MoveGenerator::new(1, KNIGHT_TRANSFORMERS.into()),
//         PieceType::Bishop => MoveGenerator::new(MAX_POSITION, DIAGONAL_TRANSFORMERS.into()),
//         PieceType::Rook => MoveGenerator::new(MAX_POSITION, linear_transformers),
//         PieceType::Queen => MoveGenerator::new(MAX_POSITION, all_transformers.clone()),
//         PieceType::King => MoveGenerator::new(1, all_transformers)
//     }
// }

//     fn new(max_movement: i8, transformers: Vec<Box<dyn Transformer>>) -> Self {
//         MoveGenerator {
//             max_movement,
//             transformers,
//         }
//     }

//     pub fn generate(&self, piece: &Piece, others: &[Piece]) -> Vec<Position> {
//         let mut positions = vec![];
//         for transformer in &self.transformers {
//             for movement in 0..self.max_movement {
//                 if let Some(position) =
//                     find_available_position(piece, others, transformer, movement)
//                 {
//                     positions.push(position)
//                 }
//             }
//         }
//         positions
//     }
// }

// fn find_available_position(
//     piece: &Piece,
//     others: &[Piece],
//     transformer: &dyn Transformer,
//     multiplier: i8,
// ) -> Option<Position> {
//     let candidate_position = transformer.apply(piece, multiplier)?;
//     let current_occupant = others.iter().find(|p| p.position() == &candidate_position);

//     match current_occupant {
//         Some(occupant) => {
//             if occupant.colour() == piece.colour() {
//                 None
//             } else {
//                 Some(candidate_position)
//             }
//         }
//         None => Some(candidate_position),
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     mod find_available_position_test {

//         use crate::engine::transformer::SimpleTransformer;

//         use super::*;

//         #[test]
//         fn returns_position_if_unoccupied() {
//             let piece = Piece::new(
//                 PieceColour::Black,
//                 PieceType::Pawn,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let transformer = SimpleTransformer::new(1, 0);
//             let position = find_available_position(&piece, &[], &transformer, 1);

//             assert_eq!(position, Some(Position::new(2, 1).unwrap()))
//         }

//         #[test]
//         fn returns_position_if_occupied_by_opposing_colour() {
//             let piece = Piece::new(
//                 PieceColour::Black,
//                 PieceType::Pawn,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let others = &[Piece::new(
//                 PieceColour::White,
//                 PieceType::Knight,
//                 Position::new(2, 2).unwrap(),
//                 false,
//             )];
//             let transformer = SimpleTransformer::new(1, 0);
//             let position = find_available_position(&piece, others, &transformer, 1);

//             assert_eq!(position, Some(Position::new(2, 1).unwrap()))
//         }

//         #[test]
//         fn returns_none_if_occupied_by_same_colour() {
//             let piece = Piece::new(
//                 PieceColour::Black,
//                 PieceType::Pawn,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let others = &[Piece::new(
//                 PieceColour::Black,
//                 PieceType::Knight,
//                 Position::new(2, 1).unwrap(),
//                 false,
//             )];
//             let transformer = SimpleTransformer::new(1, 0);
//             let position = find_available_position(&piece, others, &transformer, 1);

//             assert_eq!(position, None)
//         }

//         #[test]
//         fn returns_none_if_new_position_invalid() {
//             let piece = Piece::new(
//                 PieceColour::Black,
//                 PieceType::Pawn,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let transformer = SimpleTransformer::new(-3, -3);
//             let position = find_available_position(&piece, &[], &transformer, 1);

//             assert_eq!(position, None)
//         }
//     }
// }
