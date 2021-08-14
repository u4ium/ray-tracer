use crate::{
    ray::{Hit, Ray},
    vector::HVector,
};
use std::f64::consts::PI;

pub mod intersection;
use intersection::find_closest_intersection;

mod parsers;
//use parsers::*;

pub mod material;
use material::Material;

pub mod matrix;
use matrix::{AffineMatrix, AffineTransformation};

// Semantically allow only triangle meshes with similar mapppings for all triangles
//fn get_points(&self) -> [HVector; 3];
trait LeafShape {
    fn intersection<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>>; // TODO: no material on leaf hits
    fn get_normal(&self, u: f64, v: f64) -> HVector;
    fn get_texture_coordinates(&self, u: f64, v: f64) -> [f64; 2];
}

pub struct MappedTrianglePoint {
    point: HVector,
    texture_coordinates: [f64; 2],
    normal: HVector,
}

pub enum ObjectShape {
    Sphere,
    Triangle(HVector, HVector, HVector),
    GroupedMesh(Vec<TexturedObject>),
    Mesh(Vec<LeafObject>),
}
use ObjectShape::*;

const EPSILON: f64 = 1e-20;

impl ObjectShape {
    fn intersection<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>> {
        let material = None;
        match self {
            Sphere => {
                // ray = from + k * direction
                // sphere = x^2 + y^2 + z^2 = 1
                //      1 = ray . ray
                //        = (F + kD) . (F + kD)
                //        = (D.D) * k^2     + (2*D.F) * k       + (F.F - 1)
                //        =   a   * k^2     +  2 * b  * k       +     c
                // a = D.D = 1
                // b = D.F
                // c = F.F
                // d = b^2 - c
                // k = (-B +/- sqrt(d))
                let b = ray.direction.dot(&ray.from);
                let c = ray.from.dot(&ray.from) - 1.0;
                let d = b * b - c;
                if d < 0.0 {
                    return None; // no intersection
                }

                let k = {
                    let k1 = -b + d.sqrt();
                    let k2 = -b - d.sqrt();
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
                let [x, y, z] = hit_point.to_array();
                let texture_coordinates = [0.5 + x.atan2(z) / (2.0 * PI), 0.5 + y.asin() / PI];

                Some(Hit {
                    normal,
                    texture_coordinates,
                    material,
                })
            }
            Triangle(p1, p2, p3) => {
                // Moeller-Trombore intersection algorithm
                let side1 = p2.clone() - p1.clone();
                let side2 = p3.clone() - p1.clone();
                let p_vector = ray.direction.cross(&side2);

                let determinant = side1.dot(&p_vector);
                if determinant <= EPSILON {
                    // TODO: if double-sided, check absolute value
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

                // Compute and return Hit
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
                    material,
                })
            }
            GroupedMesh(children) => find_closest_intersection(children, ray),
            Mesh(children) => find_closest_intersection(children, ray),
        }
    }
}

//TODO: #[derive(Debug)]
pub struct Object {
    material: Material,
    matrix: AffineMatrix,
    shape: ObjectShape,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn get_material(&self) -> Option<&Material>;
}

impl Object {
    pub fn new(
        shape: ObjectShape,
        transformation: Option<AffineTransformation>,
        material: Option<Material>,
    ) -> Object {
        let matrix = AffineMatrix::new(transformation.unwrap_or_default());
        let material: Material = material.unwrap_or_default();
        Object {
            shape,
            matrix,
            material,
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape
            .intersection(&self.matrix.shift(ray))
            .and_then(|hit| {
                Some(Hit {
                    normal: self.matrix.unshift(&hit.normal),
                    material: hit.material.or(self.get_material()),
                    ..hit
                })
            })
    }

    fn get_material(&self) -> Option<&Material> {
        Some(&self.material)
    }
}

pub struct ChildObject {
    matrix: AffineMatrix,
    shape: ObjectShape,
}

impl ChildObject {
    pub fn new(shape: ObjectShape, transformation: Option<AffineTransformation>) -> ChildObject {
        let matrix = AffineMatrix::new(transformation.unwrap_or_default());
        ChildObject { shape, matrix }
    }
}

impl Intersectable for ChildObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape
            .intersection(&self.matrix.shift(ray))
            .and_then(|hit| {
                Some(Hit {
                    normal: self.matrix.unshift(&hit.normal),
                    material: hit.material.or(self.get_material()),
                    ..hit
                })
            })
    }

    fn get_material(&self) -> Option<&Material> {
        None
    }
}

pub struct LeafObject {
    shape: ObjectShape,
}

impl LeafObject {
    pub fn new(shape: ObjectShape) -> LeafObject {
        LeafObject { shape }
    }
}

impl Intersectable for LeafObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape.intersection(ray)
    }

    fn get_material(&self) -> Option<&Material> {
        None
    }
}

pub struct TexturedObject {
    shape: ObjectShape,
    material: Material,
}
impl TexturedObject {
    pub fn new(shape: ObjectShape, material: Option<Material>) -> TexturedObject {
        let material = material.unwrap_or_default();
        TexturedObject { shape, material }
    }
}

impl Intersectable for TexturedObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.shape.intersection(ray).and_then(|hit| {
            Some(Hit {
                material: hit.material.or(self.get_material()),
                ..hit
            })
        })
    }

    fn get_material(&self) -> Option<&Material> {
        Some(&self.material)
    }
}
