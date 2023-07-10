// use crate::model::{Piece, PieceColour, Position};

// pub trait Transformer {
//     fn apply(&self, piece: &Piece, multiplier: i8) -> Option<Position>;
// }

// impl<T: Transformer + ?Sized> Transformer for Box<T> {
//     fn apply(&self, piece: &Piece, multiplier: i8) -> Option<Position> {
//         (**self).apply(piece, multiplier)
//     }
// }

// pub struct SimpleTransformer {
//     x: i8,
//     y: i8,
// }

// impl SimpleTransformer {
//     pub fn new(x: i8, y: i8) -> Self {
//         SimpleTransformer { x, y }
//     }
// }

// impl Transformer for SimpleTransformer {
//     fn apply(&self, piece: &Piece, multiplier: i8) -> Option<Position> {
//         let new_row = piece.position().row() + self.x * multiplier;
//         let new_col = piece.position().col() + self.y * multiplier;
//         Position::new(new_row, new_col).ok()
//     }
// }

// pub struct FirstPawnMoveTransformer {
//     delegate: Box<dyn Transformer>,
// }

// impl FirstPawnMoveTransformer {
//     pub fn new(colour: &PieceColour) -> Self {
//         let delegate = match colour {
//             PieceColour::White => SimpleTransformer::new(2, 0),
//             PieceColour::Black => SimpleTransformer::new(-2, 0),
//         };
//         FirstPawnMoveTransformer {
//             delegate: Box::new(delegate),
//         }
//     }
// }

// impl Transformer for FirstPawnMoveTransformer {
//     fn apply(&self, piece: &Piece, multiplier: i8) -> Option<Position> {
//         match piece.has_moved() {
//             true => None,
//             false => self.delegate.apply(piece, multiplier),
//         }
//     }
// }

// struct EnPassantTransformer {}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use crate::model::PieceType;

//     struct NoOpsTransformer {}

//     impl Transformer for NoOpsTransformer {
//         fn apply(&self, piece: &Piece, _: i8) -> Option<Position> {
//             Some(piece.position().to_owned())
//         }
//     }

//     mod simple_transformer_test {

//         use super::*;

//         #[test]
//         fn returns_transformed_position_if_valid() {
//             let piece = Piece::new(
//                 PieceColour::White,
//                 PieceType::Knight,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let transformer = SimpleTransformer::new(1, 1);

//             assert_eq!(
//                 transformer.apply(&piece, 1),
//                 Some(Position::new(2, 2).unwrap())
//             )
//         }

//         #[test]
//         fn returns_none_if_new_position_invalid() {
//             let piece = Piece::new(
//                 PieceColour::White,
//                 PieceType::Knight,
//                 Position::new(1, 1).unwrap(),
//                 false,
//             );
//             let transformer = SimpleTransformer::new(-2, -2);

//             assert_eq!(transformer.apply(&piece, 1), None)
//         }
//     }

//     mod first_pawn_move_transformer_test {

//         use super::*;

//         #[test]
//         fn passes_to_delegate_if_piece_has_not_moved() {
//             let position = Position::new(1, 1).unwrap();
//             let piece = piece(position, false);
//             let transformer = transformer();

//             assert_eq!(transformer.apply(&piece, 1), Some(position))
//         }

//         #[test]
//         fn does_nothing_if_piece_has_moved() {
//             let position = Position::new(1, 1).unwrap();
//             let piece = piece(position, true);
//             let transformer = transformer();

//             assert_eq!(transformer.apply(&piece, 1), None)
//         }

//         fn piece(position: Position, has_moved: bool) -> Piece {
//             Piece::new(PieceColour::White, PieceType::Pawn, position, has_moved)
//         }

//         fn transformer() -> FirstPawnMoveTransformer {
//             FirstPawnMoveTransformer {
//                 delegate: Box::new(NoOpsTransformer {}),
//             }
//         }
//     }
// }
