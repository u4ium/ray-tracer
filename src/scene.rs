use crate::ray::Ray;
use crate::{
    image::{Pixel, BLACK},
    vector::HVector,
};
use ndarray::{array, Array2};
use std::f64::consts::PI;

pub struct AffineTransformation {
    scale: [f64; 3],
    position: [f64; 3],
    orientation: (Angle, Angle),
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
        let arz = 2.0 * PI - rz;
        let inverse_rotation_z = array![
            [arz.cos(), -arz.sin(), 0.0, 0.0],
            [arz.sin(), arz.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ary = 2.0 * PI - ry;
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

    pub fn unshift(&self, point: HVector) -> HVector {
        HVector::new(self.inverse_transpose.dot(&point.0))
    }
}

pub enum ObjectShape {
    Sphere,
    Triangle(HVector, HVector, HVector),
}
use ObjectShape::*;

pub struct Object {
    matrix: AffineMatrix,
    shape: ObjectShape,
}

impl ObjectShape {
    fn intersection(&self, ray: &Ray) -> Option<HVector> {
        match self {
            Sphere => None,
            Triangle(p1, p2, p3) => None,
        }
    }
}

type Angle = f64; //TODO

impl Object {
    pub fn new(shape: ObjectShape, transformation: AffineTransformation) -> Object {
        let matrix = AffineMatrix::new(transformation);
        Object { shape, matrix }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HVector> {
        self.shape
            .intersection(&self.matrix.shift(ray))
            .and_then(|point| Some(self.matrix.unshift(point)))
    }
    pub fn get_colour(&self, point: &HVector, direction: &HVector) -> Pixel {
        // TODO: Phong illumination model
        Pixel {
            red: 0.5,
            green: 0.0,
            blue: 0.0,
        }
    }
}

pub struct Scene {
    objects: Vec<Object>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn trace(&self, ray: &Ray, depth: u8) -> Pixel {
        let mut best: Option<(f64, &Object)> = None;
        for object in self.objects.iter() {
            match object.intersect(ray) {
                Some(point) => {
                    let distance = (point - ray.from.clone()).magnitude();
                    best = match best {
                        None => Some((distance, object)),
                        Some((d, o)) => {
                            if d > distance {
                                Some((distance, object))
                            } else {
                                Some((d, o))
                            }
                        }
                    };
                }
                None => {}
            }
        }
        match best {
            Some((_, hit)) => hit.get_colour(&point, &ray.direction),
            None => BLACK,
        }
    }
}
