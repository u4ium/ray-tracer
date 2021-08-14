use ndarray::{array, s, Array1};
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct HVector(Array1<f64>);
impl HVector {
    pub fn new(vec: [f64; 3]) -> HVector {
        HVector(array![vec[0], vec[1], vec[2], 1.0])
    }

    pub fn from_array3(vec: Array1<f64>) -> HVector {
        HVector(array![vec[0], vec[1], vec[2], 1.0])
    }

    pub fn from_array4(vec: Array1<f64>) -> HVector {
        HVector(vec)
    }

    pub fn get(&self) -> &Array1<f64> {
        &self.0
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.to_array())
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> HVector {
        self.scale(1.0 / self.magnitude())
    }

    pub fn dot(&self, rhs: &HVector) -> f64 {
        self.0.slice(s![..3]).dot(&rhs.0.slice(s![..3]))
    }

    pub fn cross(&self, rhs: &HVector) -> HVector {
        let [a1, a2, a3] = self.to_array();
        let [b1, b2, b3] = rhs.to_array();
        HVector(array![
            a2 * b3 - a3 * b2,
            a3 * b1 - a1 * b3,
            a1 * b2 - a2 * b1,
            1.0
        ])
    }

    pub fn scale(&self, factor: f64) -> HVector {
        let mut v = factor * self.0.clone();
        v[3] = 1.0;
        HVector(v)
    }

    pub fn reflect(&self, normal: &HVector) -> HVector {
        self.clone() - normal.scale(2.0 * self.dot(normal))
    }

    pub fn reverse(&self) -> HVector {
        self.scale(-1.0)
    }
}

impl Sub for HVector {
    type Output = HVector;
    fn sub(self, rhs: HVector) -> Self::Output {
        let mut v = self.0 - rhs.0;
        v[3] = 1.0;
        HVector(v)
    }
}

impl Add for HVector {
    type Output = HVector;
    fn add(self, rhs: HVector) -> Self::Output {
        let mut v = self.0 + rhs.0;
        v[3] = 1.0;
        HVector(v)
    }
}

#[derive(Clone)]
pub struct Vector3(pub Array1<f64>);
impl Vector3 {
    pub fn new(vec: [f64; 3]) -> Vector3 {
        let [x, y, z] = vec;
        Vector3(array![x, y, z])
    }
    pub fn to_homo_vector(&self) -> HVector {
        HVector::new([self.0[0], self.0[1], self.0[2]])
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3(self.0 - rhs.0)
    }
}
