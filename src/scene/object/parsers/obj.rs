/// Parse Wavefront .OBJ files
mod polygon;
use self::polygon::Polygon;
mod material;
//use material::Material;

struct Group {
    //material: Material,
    polygons: Vec<Polygon>,
}

pub struct WaveFrontObject {
    name: Option<String>,
    groups: Vec<Group>,
}

impl WaveFrontObject {
    pub fn to_object(&self) {
        // TODO
    }
}

fn parse_file(filename: &str) -> Vec<WaveFrontObject> {
    let mut result = vec![];
    // TODO
    result
}
