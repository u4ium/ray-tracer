use crate::scene::object::parsers::common::identifier::parse_identifier;
use nom::IResult;

pub fn parse_material_identifier(input: &str) -> IResult<&str, &str> {
    parse_identifier("usemtl", input)
}

#[test]
fn test_parse_material_identifier() {
    assert_eq!(
        parse_material_identifier("usemtl material0\r\nb"),
        Ok(("b", "material0"))
    );
    assert!(parse_material_identifier("use_material material0\r\nb").is_err());
}
