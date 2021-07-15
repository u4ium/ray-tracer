use crate::{
    scene::Object,
    vector::{HVector, Vector3},
};

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
    pub fn intersect(&self, object: &Object) -> Option<HVector> {
        // TODO:
        None
    }
}
