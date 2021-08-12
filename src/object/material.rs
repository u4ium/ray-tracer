use crate::image::Colour;

const EPSILON: f64 = 0.00000000001;

pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub colour: Colour,
}

impl Material {
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
