use ndarray::{array, s, Array1};
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct HVector(pub Array1<f64>);
impl HVector {
    pub fn new(vec: Array1<f64>) -> HVector {
        assert!(vec.len() == 4);
        // TODO: assert!(vec[3] == 1.0);
        HVector(vec)
    }

    pub fn to_vector3(&self) -> Vector3 {
        let x = self.0[0]; // TODO: divide by self.0[4]
        let y = self.0[1]; // TODO: divide by self.0[4]
        let z = self.0[2]; // TODO: divide by self.0[4]
        Vector3::new(array![x, y, z])
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> HVector {
        self.scale(self.magnitude())
    }

    pub fn dot(&self, rhs: &HVector) -> f64 {
        self.0.slice(s![..3]).dot(&rhs.0.slice(s![..3]))
    }

    pub fn scale(&self, factor: f64) -> HVector {
        let mut v = factor * self.0.clone();
        v[3] = 1.0;
        HVector(v)
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
    pub fn new(vec: Array1<f64>) -> Vector3 {
        assert!(vec.len() == 3);
        Vector3(vec)
    }
    pub fn to_homo_vector(&self) -> HVector {
        let x = self.0[0];
        let y = self.0[1];
        let z = self.0[2];
        HVector::new(array![x, y, z, 1.0])
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let mut v = self.0 - rhs.0;
        v[3] = 1.0;
        Vector3(v)
    }
}
