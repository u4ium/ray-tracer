use crate::object::parsers::common::{float::parse_float, whitespace::tws};
use nom::{bytes::complete::tag, combinator::opt, sequence::tuple, IResult};

#[derive(Debug, PartialEq)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

fn parse_vertex(input: &str) -> IResult<&str, Vertex> {
    let (input, _) = tws(tag("v"))(input)?;
    let (input, (x, y, z, w)) = tuple((
        tws(parse_float),
        tws(parse_float),
        tws(parse_float),
        opt(tws(parse_float)),
    ))(input)?;
    let w = w.unwrap_or(1.0);

    Ok((input, Vertex { x, y, z, w }))
}

#[test]
fn test_parse_vertex() {
    assert_eq!(
        parse_vertex("v 0.123 -0.234 2.345 1.0\r\n"),
        Ok((
            "",
            Vertex {
                x: 0.123,
                y: -0.234,
                z: 2.345,
                w: 1.0,
            }
        ))
    );
}
