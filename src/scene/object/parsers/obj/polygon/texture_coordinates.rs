use crate::scene::object::parsers::common::{float::parse_float, whitespace::tws};
use nom::{bytes::complete::tag, combinator::opt, sequence::tuple, IResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TextureCoordinates {
    u: f64,
    v: f64,
    w: f64,
}

fn parse_texture_coordinates(input: &str) -> IResult<&str, TextureCoordinates> {
    let (input, _) = tws(tag("vt"))(input)?;
    let (input, (u, v, w)) = tuple((
        tws(parse_float),
        opt(tws(parse_float)),
        opt(tws(parse_float)),
    ))(input)?;
    let v = v.unwrap_or(0.0);
    let w = w.unwrap_or(0.0);
    // TODO: Only accept values in range [0.0, 1.0], else -> ERROR
    for n in [u, v, w] {
        if n < 0.0 {}
        if n > 1.0 {}
    }
    Ok((input, TextureCoordinates { u, v, w }))
}

#[test]
fn test_parse_texture_coordinates() {
    assert_eq!(
        parse_texture_coordinates("vt 0.500 0"),
        Ok((
            "",
            TextureCoordinates {
                u: 0.5,
                v: 0.0,
                w: 0.0,
            }
        ))
    );
    assert_eq!(
        parse_texture_coordinates("vt 0.500 1 1\r\n"),
        Ok((
            "",
            TextureCoordinates {
                u: 0.5,
                v: 1.0,
                w: 1.0,
            }
        ))
    );
}
