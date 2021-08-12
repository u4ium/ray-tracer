use nom::{character::complete::multispace0, error::ParseError, sequence::terminated, IResult};

/// A combinator that takes a parser `inner` and produces a parser that also consumes
/// trailing whitespace, returning the output of `inner`.
pub fn tws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, multispace0)
}
