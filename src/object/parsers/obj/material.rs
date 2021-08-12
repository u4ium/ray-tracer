use crate::object::parsers::common::identifier::parse_identifier;
use nom::IResult;

pub mod filename;
pub mod identifier;
pub mod map;

//TODO????????
enum FileType {
    Material,
    TextureMap,
}

struct FilenameParser {
    extension: &'static str,
    file_type: FileType,
}

// Trait bound
//fn parse_file_name(input: &str) -> IResult<&str, &str>;
