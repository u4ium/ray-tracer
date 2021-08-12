use crate::object::parsers::common::{float::parse_float, whitespace::tws};
use nom::{bytes::complete::tag, sequence::tuple, IResult};

#[derive(Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn parse_vector(input: &str) -> IResult<&str, Vector> {
    let (input, _) = tws(tag("vn"))(input)?;
    let (input, (x, y, z)) = tuple((tws(parse_float), tws(parse_float), tws(parse_float)))(input)?;
    Ok((input, Vector { x, y, z }))
}

#[test]
fn test_parse_vector() {
    assert_eq!(
        parse_vector("vn 0.123 -0.234 2.345 "),
        Ok((
            "",
            Vector {
                x: 0.123,
                y: -0.234,
                z: 2.345,
            }
        ))
    );
    //
    assert_eq!(
        parse_vector("vn 0.707 0.000 0.707\r\n"),
        Ok((
            "",
            Vector {
                x: 0.707,
                y: 0.0,
                z: 0.707,
            }
        ))
    );
}
