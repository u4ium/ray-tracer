use crate::scene::object::parsers::common::integer::decimal;
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

fn float(input: &str) -> IResult<&str, &str> {
    recognize(preceded(
        opt(char('-')),
        alt((
            // Case one: ".42"
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))),
            // Case two: "42e42" and "42.42e42"
            recognize(tuple((
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))),
            // Case three: "42". and "42.42"
            recognize(tuple((decimal, char('.'), opt(decimal)))),
            // Case four: "42"
            decimal,
        )),
    ))(input)
}

#[test]
fn test_float() {
    assert_eq!(float(".42"), Ok(("", ".42")));
    assert_eq!(float("1."), Ok(("", "1.")));
    assert_eq!(float("-7.42e+13b"), Ok(("b", "-7.42e+13")));
    assert_eq!(float("3.42e-15 "), Ok((" ", "3.42e-15")));
}

fn string_to_float(input: &str) -> f64 {
    input.parse::<f64>().unwrap() // TODO: add error-hanlding
}

pub fn parse_float(input: &str) -> IResult<&str, f64> {
    map(float, string_to_float)(input)
}

#[test]
fn test_parse_float() {
    assert_eq!(parse_float("0.187 "), Ok((" ", 0.187)));
    assert_eq!(parse_float("-3.14159"), Ok(("", -3.14159)));
    assert_eq!(parse_float("0.187e"), Ok(("e", 0.187)));
}
