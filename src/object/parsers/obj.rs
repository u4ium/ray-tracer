/// Parse Wavefront .OBJ files
mod polygon;
use self::polygon::Polygon;
mod material;

pub struct WaveFrontObject {
    polygons: Vec<Polygon>,
}

fn parse_file(filename: &str) -> Vec<WaveFrontObject> {
    let mut result = vec![];
    // TODO
    result
}
