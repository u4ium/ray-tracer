use crate::scene::object::parsers::common::whitespace::tws;
use nom::{
    bytes::complete::{is_not, tag, tag_no_case, take_while, take_while1},
    combinator::{all_consuming, recognize},
    multi::many1,
    multi::many_till,
    sequence::{preceded, tuple},
    IResult,
};

pub fn recognize_filename<'a>(
    extension: &'static str,
    input: &'a str,
) -> IResult<&'a str, &'a str> {
    let (input, matched) = tws(is_not(" \t"))(input)?;
    all_consuming(recognize(tuple((
        take_while(|c| c == '.' || c == '/'),
        take_while1(|c| c != '.'),
        recognize(many_till(take_while(|_| true), tag_no_case(extension))),
    ))))(matched)?;
    Ok((input, matched))
}

pub fn parse_file_name<'a, F>(
    tag_string: &'static str,
    recognize_filename: F,
    input: &'a str,
) -> IResult<&'a str, &'a str>
where
    F: Fn(&str) -> IResult<&str, &str>,
{
    let (input, filename) = preceded(tws(tag(tag_string)), tws(is_not("\r\n")))(input)?;
    let (_, filename) = recognize_filename(filename)?;
    Ok((input, filename))
}

pub fn parse_filenames<'a, F>(
    tag_string: &'static str,
    recognize_filename: F,
    input: &'a str,
) -> IResult<&'a str, Vec<&'a str>>
where
    F: Fn(&str) -> IResult<&str, &str>,
{
    let (input, filenames) = preceded(tws(tag(tag_string)), tws(is_not("\r\n")))(input)?;
    let (_, filenames) = many1(recognize_filename)(filenames)?;
    Ok((input, filenames))
}
