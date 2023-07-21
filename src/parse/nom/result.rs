use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map_res},
    IResult,
};

use crate::model::GameResult;

use super::error::PgnParseError;

pub fn parse_result(input: &str) -> IResult<&str, GameResult> {
    let parser = all_consuming(alt((tag("1-0"), tag("0-1"), tag("1/2-1/2"), tag("*"))));
    map_res(parser, |result| match result {
        "1-0" => Ok(GameResult::WhiteWin),
        "0-1" => Ok(GameResult::BlackWin),
        "1/2-1/2" => Ok(GameResult::Draw),
        "*" => Ok(GameResult::Ongoing),
        _ => Err(PgnParseError::new(format!(
            "'{result}' is not a valid game result"
        ))),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_err_if_not_game_result() {
        let result = parse_result("something");
        assert!(result.is_err())
    }

    #[test]
    fn returns_err_if_any_input_remains() {
        let result = parse_result("1-0 something");
        assert!(result.is_err())
    }

    #[test]
    fn parses_result() {
        let result = parse_result("0-1").unwrap();
        assert_eq!(result, ("", GameResult::BlackWin))
    }
}
