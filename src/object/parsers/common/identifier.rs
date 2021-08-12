use crate::object::parsers::common::whitespace::tws;
use nom::{
    bytes::complete::{is_not, tag},
    sequence::preceded,
    IResult,
};

pub fn parse_identifier<'a>(tag_string: &'static str, input: &'a str) -> IResult<&'a str, &'a str> {
    preceded(tws(tag(tag_string)), tws(is_not("\r\n")))(input)
}
