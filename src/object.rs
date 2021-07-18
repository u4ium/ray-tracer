use crate::ray::{Hit, Ray};
use crate::vector::HVector;

pub mod material;
use material::Material;

pub mod matrix;
use matrix::{AffineMatrix, AffineTransformation};

pub enum ObjectShape {
    Sphere,
    Triangle(HVector, HVector, HVector),
}
use ObjectShape::*;

pub struct Object {
    matrix: AffineMatrix,
    shape: ObjectShape,
    material: Material,
}

const EPSILON: f64 = 0.000000001;

impl ObjectShape {
    fn intersection(&self, ray: &Ray) -> Option<Hit> {
        const NULL_COORDINATES: [f64; 2] = [0.0, 0.0]; // TODO
        match self {
            Sphere => {
                // ray = from + k * direction
                // sphere = x^2 + y^2 + z^2 = 1
                //      1 = ray . ray
                //        = (F + kD) . (F + kD)
                //        = D.D*k^2 + + 2*D.F*k + F.F - 1
                //        = Ak^2 + Bk + C
                // A = 1 if normalized
                // B = 2 * D.F
                // C = F.F
                // d = B^2 - 4C
                // k = (-B +/- sqrt(d)) / 2A
                let b = 2.0 * ray.direction.dot(&ray.from);
                let c = ray.from.dot(&ray.from) - 1.0;
                let d = b * b - 4.0 * c;
                if d < 0.0 {
                    return None; // no intersection
                }
                let k = {
                    let k1 = (-b + d.sqrt()) / 2.0;
                    let k2 = (-b - d.sqrt()) / 2.0;
                    if k2 <= EPSILON {
                        k1
                    } else {
                        k2
                    }
                };
                if k <= EPSILON {
                    None // hit object too close or backwards
                } else {
                    let hit_point = ray.from.clone() + ray.direction.scale(k);
                    let normal = Ray {
                        from: hit_point.clone(),
                        direction: hit_point,
                    };
                    Some(Hit {
                        normal,
                        texture_coordinates: NULL_COORDINATES,
                    })
                }
            }
            Triangle(_p1, _p2, _p3) => None, // TODO
        }
    }
}

impl Object {
    pub fn new(
        shape: ObjectShape,
        transformation: AffineTransformation,
        material: Material,
    ) -> Object {
        let matrix = AffineMatrix::new(transformation);
        Object {
            shape,
            matrix,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape
            .intersection(&self.matrix.shift(ray))
            .and_then(|hit| {
                Some(Hit {
                    normal: self.matrix.unshift(&hit.normal),
                    ..hit
                })
            })
    }

    pub fn get_material(&self) -> &Material {
        &self.material // TODO: replace with get_colour
    }
}
