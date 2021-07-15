use ndarray::{array, Array2};

use crate::ray::Ray;
use crate::vector::HVector;

type Angle = f64; //TODO

pub struct AffineTransformation {
    pub scale: [f64; 3],
    pub position: [f64; 3],
    pub orientation: (Angle, Angle),
}

pub struct AffineMatrix {
    actual: Array2<f64>,
    inverse_transpose: Array2<f64>,
}

impl AffineMatrix {
    pub fn new(transformation: AffineTransformation) -> AffineMatrix {
        let scale = transformation.scale;
        let (ry, rz) = transformation.orientation;
        let position = transformation.position;
        let scaling = array![
            [scale[0], 0.0, 0.0, 0.0],
            [0.0, scale[1], 0.0, 0.0],
            [0.0, 0.0, scale[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation_y = array![
            [ry.cos(), 0.0, ry.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-ry.sin(), 0.0, ry.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation_z = array![
            [rz.cos(), -rz.sin(), 0.0, 0.0],
            [rz.sin(), rz.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rotation = rotation_z.dot(&rotation_y);
        let translation = array![
            [1.0, 0.0, 0.0, position[0]],
            [0.0, 1.0, 0.0, position[1]],
            [0.0, 0.0, 1.0, position[2]],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let actual = translation.dot(&rotation).dot(&scaling);

        let inverse_scaling = array![
            [1.0 / scale[0], 0.0, 0.0, 0.0],
            [0.0, 1.0 / scale[1], 0.0, 0.0],
            [0.0, 0.0, 1.0 / scale[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let inverse_translation = array![
            [1.0, 0.0, 0.0, -position[0]],
            [0.0, 1.0, 0.0, -position[1]],
            [0.0, 0.0, 1.0, -position[2]],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let arz = -rz;
        let inverse_rotation_z = array![
            [arz.cos(), -arz.sin(), 0.0, 0.0],
            [arz.sin(), arz.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ary = -ry;
        let inverse_rotation_y = array![
            [ary.cos(), 0.0, ary.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-ary.sin(), 0.0, ary.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let inverse_rotation = inverse_rotation_y.dot(&inverse_rotation_z);
        let inverse = inverse_scaling.dot(&inverse_rotation.dot(&inverse_translation));
        let inverse_transpose = inverse.reversed_axes();

        AffineMatrix {
            actual,
            inverse_transpose,
        }
    }

    pub fn shift_point(&self, point: &HVector) -> HVector {
        HVector::new(self.actual.dot(&point.0))
    }

    pub fn shift(&self, ray: &Ray) -> Ray {
        Ray {
            from: self.shift_point(&ray.from),
            direction: self.shift_point(&ray.direction),
        }
    }

    pub fn unshift_point(&self, point: &HVector) -> HVector {
        HVector::new(self.inverse_transpose.dot(&point.0))
    }

    pub fn unshift(&self, ray: &Ray) -> Ray {
        Ray {
            from: self.unshift_point(&ray.from),
            direction: self.unshift_point(&ray.direction),
        }
    }
}
