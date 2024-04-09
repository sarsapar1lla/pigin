use crate::model::Board;
use crate::model::PieceColour;
use crate::model::PieceType;

pub fn halfmove(board: &mut Board, piece_type: PieceType, capture: bool) -> &mut Board {
    if piece_type == PieceType::Pawn || capture {
        board.update_halfmove_clock(0);
    } else {
        board.update_halfmove_clock(board.halfmove_clock() + 1);
    }
    board
}

pub fn fullmove(board: &mut Board, active_colour: PieceColour) -> &mut Board {
    if active_colour == PieceColour::Black {
        board.update_fullmove_clock(board.fullmove_clock() + 1);
    }
    board
}
