use crate::model::Board;
use crate::model::PieceColour;

pub fn update(board: &mut Board) -> &mut Board {
    let next_active_colour = match board.active_colour() {
        PieceColour::White => PieceColour::Black,
        PieceColour::Black => PieceColour::White,
    };

    board.update_active_colour(next_active_colour);
    board
}
