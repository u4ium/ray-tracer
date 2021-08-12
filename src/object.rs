use crate::ray::{Hit, Ray};
use crate::vector::HVector;
use std::f64::consts::PI;
mod parsers;
//use parsers::*;

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

const EPSILON: f64 = 1e-20;

impl ObjectShape {
    fn intersection(&self, ray: &Ray) -> Option<Hit> {
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
                    return None; // hit object too close or backwards
                }

                let hit_point = ray.from.clone() + ray.direction.scale(k);
                let normal = Ray {
                    from: hit_point.clone(),
                    direction: hit_point.clone(),
                };
                let (x, y, _) = hit_point.to_3tuple();
                let texture_coordinates = [x.asin() / PI + 0.5, y.asin() / PI + 0.5];

                Some(Hit {
                    normal,
                    texture_coordinates,
                })
            }
            Triangle(p1, p2, p3) => {
                let side1 = p2.clone() - p1.clone();
                let side2 = p3.clone() - p1.clone();
                let p_vector = ray.direction.cross(&side2);

                let determinant = side1.dot(&p_vector);
                if determinant.abs() <= EPSILON {
                    // TODO: if not double-sided, check signed value (fix direction)
                    return None; // parallel to or hit wrong side of triangle
                }
                let inverse_determinant = 1.0 / determinant;

                let t_vector = ray.from.clone() - p1.clone();
                let u = t_vector.dot(&p_vector) * inverse_determinant;
                if u < 0.0 || u > 1.0 {
                    return None; // barycentric coordinates not in triangle
                }

                let q_vector = t_vector.cross(&side1);
                let v = ray.direction.dot(&q_vector) * inverse_determinant;
                if v < 0.0 || u + v > 1.0 {
                    return None; // barycentric coordinates not in triangle
                }

                let distance = side2.dot(&q_vector) * inverse_determinant;
                if distance <= EPSILON {
                    return None; // hit object too close or backwards
                }

                let plane_normal = side1.cross(&side2);
                let hit_point = ray.from.clone() + ray.direction.scale(distance);
                let normal = Ray {
                    from: hit_point,
                    direction: plane_normal,
                };
                let texture_coordinates = [u, v];

                Some(Hit {
                    normal,
                    texture_coordinates,
                })
            } // TODO
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
