use ndarray::array;

use crate::{
    image::{Pixel, WHITE},
    vector::HVector,
};
enum LightShape {
    PointLight,
}

pub struct Light {
    shape: LightShape,
    location: HVector,
    colour: Pixel,
}

impl Light {
    pub fn new(point: [f64; 3]) -> Light {
        Light {
            shape: LightShape::PointLight,
            location: HVector::new(array![point[0], point[1], point[2]]),
            colour: WHITE,
        }
    }

    pub fn direction_from(&self, point: &HVector) -> HVector {
        (self.location.clone() - point.clone()).normalized()
    }

    pub fn get_colour(&self) -> Pixel {
        self.colour
    }
}
