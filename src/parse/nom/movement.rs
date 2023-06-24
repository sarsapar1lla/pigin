use crate::model::Ply;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, line_ending, space1};
use nom::combinator::map;
use nom::sequence::delimited;
use nom::{
    character::complete::{digit1, space0},
    combinator::map_res,
    sequence::{terminated, tuple},
    IResult,
};

fn parse_moves(input: &str) -> IResult<&str, Vec<Ply>> {
    todo!()
}

// fn parse_move(input: &str) -> IResult<&str, Vec<Ply>> {
//     tuple((move_number, ply, comment, ply, comment))(input)
// }

fn move_number(input: &str) -> IResult<&str, u8> {
    map_res(terminated(digit1, tuple((char('.'), space0))), |s: &str| {
        s.parse::<u8>()
    })(input)
}

fn comment(input: &str) -> IResult<&str, String> {
    alt((parenthesis_comment, semicolon_comment))(input)
}

fn parenthesis_comment(input: &str) -> IResult<&str, String> {
    let parser = terminated(delimited(char('{'), take_until("}"), char('}')), space1);
    map(parser, |s: &str| s.replace('\n', " "))(input)
}

fn semicolon_comment(input: &str) -> IResult<&str, String> {
    let parser = delimited(char(';'), take_until("\n"), line_ending);
    map(parser, |s: &str| s.trim().to_string())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod move_number_tests {
        use super::*;

        #[test]
        fn parses_move_number_with_space() {
            let result = move_number("1. e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }

        #[test]
        fn parses_move_number_without_space() {
            let result = move_number("1.e4").unwrap();
            assert_eq!(result, ("e4", 1))
        }
    }

    mod comment_tests {
        use super::*;

        #[test]
        fn parses_comment_in_parenthesis() {
            let result = comment("{Comment} d5").unwrap();
            assert_eq!(result, ("d5", "Comment".to_string()))
        }

        #[test]
        fn parses_multiline_comment_in_parenthesis() {
            let result = comment("{Comment\ncontinued} d5").unwrap();
            assert_eq!(result, ("d5", "Comment continued".to_string()))
        }

        #[test]
        fn parses_semicolon_comment() {
            let result = comment("; This comment runs to the end of the line\nd5").unwrap();
            assert_eq!(
                result,
                ("d5", "This comment runs to the end of the line".to_string())
            )
        }
    }
}
