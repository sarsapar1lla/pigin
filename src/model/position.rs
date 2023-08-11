pub const MIN_POSITION: i8 = 0;
pub const MAX_POSITION: i8 = 7;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidPositionError(String);

impl InvalidPositionError {
    pub fn new(message: String) -> Self {
        InvalidPositionError(message)
    }
}

impl std::fmt::Display for InvalidPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidPositionError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    row: i8,
    col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        if is_invalid(row, col) {
            panic!("Invalid position ({row}, {col})");
        } else {
            Position { row, col }
        }
    }

    pub fn try_from(row: i8, col: i8) -> Result<Self, InvalidPositionError> {
        if is_invalid(row, col) {
            return Err(InvalidPositionError(format!(
                "Invalid position: ({row}, {col})"
            )));
        }
        Ok(Position { row, col })
    }

    pub fn row(&self) -> i8 {
        self.row
    }

    pub fn col(&self) -> i8 {
        self.col
    }
}

fn is_invalid(row: i8, col: i8) -> bool {
    !(MIN_POSITION..=MAX_POSITION).contains(&row) || !(MIN_POSITION..=MAX_POSITION).contains(&col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_position_if_valid() {
        let position = Position::try_from(1, 1);
        assert_eq!(position, Ok(Position { row: 1, col: 1 }))
    }

    #[test]
    fn returns_error_if_position_is_invalid() {
        let row_below_minimum = Position::try_from(-1, 3);
        let row_above_maximum = Position::try_from(8, 3);
        let col_below_minimum = Position::try_from(3, -1);
        let col_above_maximum = Position::try_from(3, 8);

        assert_eq!(
            row_below_minimum,
            Err(InvalidPositionError(
                "Invalid position: (-1, 3)".to_string()
            ))
        );
        assert_eq!(
            row_above_maximum,
            Err(InvalidPositionError("Invalid position: (8, 3)".to_string()))
        );
        assert_eq!(
            col_below_minimum,
            Err(InvalidPositionError(
                "Invalid position: (3, -1)".to_string()
            ))
        );
        assert_eq!(
            col_above_maximum,
            Err(InvalidPositionError("Invalid position: (3, 8)".to_string()))
        );
    }
}
