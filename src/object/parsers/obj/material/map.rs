use crate::object::parsers::common::{
    filename::{parse_filenames, recognize_filename},
    identifier::parse_identifier,
};
use nom::IResult;

fn recognize_map_filename(input: &str) -> IResult<&str, &str> {
    recognize_filename(".ppm", input)
}

pub fn parse_map_filenames(input: &str) -> IResult<&str, Vec<&str>> {
    parse_filenames("maplib", recognize_map_filename, input)
}

pub fn parse_texture_map_identifier(input: &str) -> IResult<&str, &str> {
    parse_identifier("usemap", input)
}

#[test]
fn test_parse_texture_map_identifier() {
    assert_eq!(
        parse_texture_map_identifier("usemap material0\r\nb"),
        Ok(("b", "material0"))
    );
    assert_eq!(parse_texture_map_identifier("usemap off"), Ok(("", "off")));
    assert!(parse_texture_map_identifier("usemtl material0\r\nb").is_err());
}
