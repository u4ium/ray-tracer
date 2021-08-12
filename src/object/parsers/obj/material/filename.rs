use crate::object::parsers::common::filename::{parse_filenames, recognize_filename};
use nom::IResult;

fn recognize_material_filename(input: &str) -> IResult<&str, &str> {
    recognize_filename(".mtl", input)
}

#[test]
fn test_recognize_material_filename() {
    // TODO: test with accidental dots after valid path.mtl
    assert_eq!(
        recognize_material_filename("../a/ab.mtl"),
        Ok(("", "../a/ab.mtl"))
    );
    assert_eq!(
        recognize_material_filename("../a/ab.mtl "),
        Ok(("", "../a/ab.mtl"))
    );
    assert_eq!(
        recognize_material_filename("subfolder/material.MTL ./folder/filename.MtL"),
        Ok(("./folder/filename.MtL", "subfolder/material.MTL"))
    );
    assert_eq!(
        recognize_material_filename("filename.MtL"),
        Ok(("", "filename.MtL"))
    );
    assert_eq!(
        recognize_material_filename("./folder/filename.MtL"),
        Ok(("", "./folder/filename.MtL"))
    );
    // TODO: allow hidden folders and filenames with dots
    // assert!(recognize_material_filename(".filename.mtl").is_err());
    assert!(recognize_material_filename("./folder/.filename.mtl").is_err());
    assert!(recognize_material_filename(".mtl").is_err());
    assert!(recognize_material_filename("././mtl").is_err());
    assert!(recognize_material_filename("././.mtl").is_err());
    assert!(recognize_material_filename("/.mtl").is_err());
}

pub fn parse_material_filenames(input: &str) -> IResult<&str, Vec<&str>> {
    parse_filenames("mtllib", recognize_material_filename, input)
}

#[test]
fn test_parse_material_filenames() {
    assert_eq!(
        parse_material_filenames("mtllib Super_Mario.mtl"),
        Ok(("", vec!["Super_Mario.mtl"]))
    );
    assert_eq!(
        parse_material_filenames("mtllib .Super_Mario.mtl\r\ng"),
        Ok(("g", vec![".Super_Mario.mtl"]))
    );
    assert_eq!(
        parse_material_filenames("mtllib subfolder/material.MTL\r\n.mtl"),
        Ok((".mtl", vec!["subfolder/material.MTL"]))
    );
    assert_eq!(
        parse_material_filenames("mtllib subfolder/material.mtl material.mtl\nnext line"),
        Ok(("next line", vec!["subfolder/material.mtl", "material.mtl"]))
    );
    assert_eq!(
        parse_material_filenames(
            "mtllib subfolder/material.MTL ./folder/filename.MtL\r\nnext line"
        ),
        Ok((
            "next line",
            vec!["subfolder/material.MTL", "./folder/filename.MtL"]
        ))
    );
    assert!(parse_material_filenames("mtllib Super_Mario.mtr\r\ng").is_err());
    assert!(parse_material_filenames("mtllib Super_Mario.mtlg").is_err());
    assert!(parse_material_filenames("mtlib Super_Mario.mtl\r\ng").is_err());
}
