use crate::object::parsers::common::whitespace::tws;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until, take_while1},
    combinator::{all_consuming, recognize},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

fn recognize_material_filename(input: &str) -> IResult<&str, &str> {
    let (input, matched) = tws(is_not(" \t"))(input)?;
    all_consuming(recognize(tuple((
        alt((
            // TODO: There must be a better way of doing this with Nom
            take_until(".mtl"),
            take_until(".mtL"),
            take_until(".mTl"),
            take_until(".mTL"),
            take_until(".Mtl"),
            take_until(".MtL"),
            take_until(".MTl"),
            take_until(".MTL"),
        )),
        tag_no_case(".mtl"),
    ))))(matched)?;
    Ok((input, matched))
}

pub fn parse_material_filename(input: &str) -> IResult<&str, Vec<&str>> {
    // TODO: allow multiple filenames
    let (input, filenames) = preceded(tws(tag("mtllib")), tws(is_not("\r\n")))(input)?;
    let (_, filenames) = many1(recognize_material_filename)(filenames)?;
    Ok((input, filenames))
}

#[test]
fn test_parse_material_filename() {
    assert_eq!(
        parse_material_filename("mtllib Super_Mario.mtl"),
        Ok(("", vec!["Super_Mario.mtl"]))
    );
    assert_eq!(
        parse_material_filename("mtllib .Super_Mario.mtl\r\ng"),
        Ok(("g", vec![".Super_Mario.mtl"]))
    );
    assert_eq!(
        parse_material_filename("mtllib subfolder/material.MTL\r\n.mtl"),
        Ok((".mtl", vec!["subfolder/material.MTL"]))
    );
    assert_eq!(
        parse_material_filename("mtllib subfolder/material.mtl material.mtl\r\nnext line"),
        Ok(("next line", vec!["subfolder/material.mtl", "material.mtl"]))
    );
    assert_eq!(
        parse_material_filename(
            "mtllib subfolder/material.MTL ./stupid-folder/.stupid.filename.MtL\r\nnext line"
        ),
        Ok((
            "next line",
            vec![
                "subfolder/material.MTL",
                "./stupid-folder/.stupid.filename.MtL"
            ]
        ))
    );
    assert!(parse_material_filename("mtllib Super_Mario.mtr\r\ng").is_err());
    assert!(parse_material_filename("mtllib Super_Mario.mtlg").is_err());
    assert!(parse_material_filename("mtlib Super_Mario.mtl\r\ng").is_err());
}

pub fn parse_material_identifier(input: &str) -> IResult<&str, &str> {
    preceded(tws(tag("usemtl")), tws(is_not("\r\n")))(input)
}

#[test]
fn test_parse_material_identifier() {
    assert_eq!(
        parse_material_identifier("usemtl material0\r\nb"),
        Ok(("b", "material0"))
    );
    assert!(parse_material_identifier("use_material material0\r\nb").is_err());
}
