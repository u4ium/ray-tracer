use nom::{
    bytes::complete::is_not,
    character::complete::{char, multispace0},
    combinator::value,
    sequence::{pair, terminated},
    IResult,
};

pub fn parse_eol_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        pair(char('#'), terminated(is_not("\n\r"), multispace0)),
    )(i)
}

#[test]
fn test_parse_eol_comment() {
    assert_eq!(
        parse_eol_comment("# This is a comment\r\nAnd now it's over"),
        Ok(("And now it's over", ()))
    );
    assert_eq!(
        parse_eol_comment("# This is a comment\r\n\r\nAnd now it's over"),
        Ok(("And now it's over", ()))
    );
    assert_eq!(
        parse_eol_comment("#This is a comment\r\n\r\n#This is another"),
        Ok(("#This is another", ()))
    );
}
