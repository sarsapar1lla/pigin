#[derive(Debug, PartialEq, Eq)]
pub struct PositionParseError(String);

impl std::fmt::Display for PositionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PositionParseError {}

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    row: i8,
    col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        Position { row, col }
    }

    pub fn row(&self) -> i8 {
        self.row
    }

    pub fn col(&self) -> i8 {
        self.col
    }
}
