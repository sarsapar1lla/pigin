use nom::character::complete::char;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    multi::many_till,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

pub fn parse_tags(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, (tags, _)) = many_till(parse_tag, tag("\n"))(input)?;
    Ok((input, tags))
}

fn parse_tag(input: &str) -> IResult<&str, (&str, &str)> {
    fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(take_until(" "), char(' '), parse_value)(input)
    }
    terminated(
        delimited(char('['), parse_key_value, char(']')),
        line_ending,
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tag_test() {
        let result = parse_tag("[Tag \"Value\"]\n1.e4").unwrap();
        assert_eq!(result, ("1.e4", ("Tag", "Value")))
    }

    #[test]
    fn parse_value_test() {
        let result = parse_value(r#""Value""#).unwrap();
        assert_eq!(result, ("", "Value"))
    }
}
