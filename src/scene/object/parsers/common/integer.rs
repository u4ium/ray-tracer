use nom::{
    character::complete::{char, one_of},
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

#[test]
fn test_decimal() {
    assert_eq!(decimal("0 "), Ok((" ", "0")));
    assert_eq!(decimal("5 "), Ok((" ", "5")));
    assert_eq!(decimal("75_000"), Ok(("", "75_000")));
    assert!(decimal("-1").is_err());
    assert!(decimal("-1").is_err());
    assert!(decimal(".1").is_err());
    assert!(decimal("_1").is_err());
}

fn string_to_usize(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

pub fn parse_integer(input: &str) -> IResult<&str, usize> {
    map(decimal, string_to_usize)(input)
}

#[test]
fn test_parse_integer() {
    assert_eq!(parse_integer("0\r\n"), Ok(("\r\n", 0)));
    assert_eq!(parse_integer("5 "), Ok((" ", 5)));
    assert_eq!(parse_integer("14000"), Ok(("", 14000)));
}
