use crate::image::Colour;
use std::default::Default;

const EPSILON: f64 = 0.00000000001;

pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub colour: Colour,
}

impl Material {
    pub const DEFAULT: Material = Material {
        ambient: 1.0 / 3.0,
        diffuse: 1.0 / 3.0,
        specular: 1.0 / 3.0,
        shininess: 4.0,
        colour: Colour::WHITE,
    };

    pub fn new(
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        colour: Colour,
    ) -> Material {
        let total = ambient + diffuse + specular;
        assert!((total - 1.0).abs() <= EPSILON);
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
            colour,
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::DEFAULT
    }
}
