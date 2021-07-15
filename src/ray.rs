use crate::vector::{HVector, Vector3};

pub struct Ray {
    pub from: HVector,
    pub direction: HVector,
}

impl Ray {
    pub fn new(from: Vector3, direction: Vector3) -> Ray {
        Ray {
            from: from.to_homo_vector(),
            direction: direction.to_homo_vector(),
        }
    }
}

pub struct Hit {
    pub normal: Ray,
    pub texture_coordinates: [f64; 2],
}
