use nom::{character::complete::one_of, combinator::map_res, sequence::pair, IResult};

use crate::model::Position;

use super::error::PgnParseError;

const ROWS: &str = "12345678";
const COLUMNS: &str = "abcdefgh";

pub fn parse(input: &str) -> IResult<&str, Position> {
    let parser = pair(column, row);
    map_res(parser, |position| {
        Position::try_from(position.1, position.0)
            .map_err(|e| PgnParseError::new(format!("Failed to parse position: {e}")))
    })(input)
}

pub fn column(input: &str) -> IResult<&str, i8> {
    map_res(one_of("abcdefgh"), |c: char| {
        COLUMNS
            .find(c)
            .map(|i| i8::try_from(i).map_err(|e| PgnParseError::new(e.to_string())))
            .transpose()?
            .ok_or_else(|| PgnParseError::new(format!("'{c}' is not a valid column")))
    })(input)
}

pub fn row(input: &str) -> IResult<&str, i8> {
    map_res(one_of("12345678"), |c: char| {
        ROWS.find(c)
            .map(|i| i8::try_from(i).map_err(|e| PgnParseError::new(e.to_string())))
            .transpose()?
            .ok_or_else(|| PgnParseError::new(format!("'{c}' is not a valid row")))
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod position_tests {
        use super::*;

        #[test]
        fn returns_err_if_not_a_position() {
            let result = parse("O-O-O e4");
            assert!(result.is_err())
        }

        #[test]
        fn parses_position() {
            let result = parse("e4 Nc5").unwrap();
            assert_eq!(result, (" Nc5", Position::try_from(3, 4).unwrap()))
        }
    }

    mod column_tests {
        use super::*;

        #[test]
        fn returns_err_if_invalid_column() {
            let result = column("j2");
            assert!(result.is_err())
        }

        #[test]
        fn parses_column() {
            let result = column("e4").unwrap();
            assert_eq!(result, ("4", 4))
        }
    }

    mod row_tests {
        use super::*;

        #[test]
        fn returns_err_if_invalid_row() {
            let result = row("b9");
            assert!(result.is_err())
        }

        #[test]
        fn parses_row() {
            let result = row("4 b5").unwrap();
            assert_eq!(result, (" b5", 3))
        }
    }
}
