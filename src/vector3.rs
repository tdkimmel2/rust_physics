use std::ops::{Mul, Div};

pub struct Vector3 {
    pub v1: f64,
    pub v2: f64,
    pub v3: f64,
    _private: (),
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3{ v1: self.v1*_rhs, v2: self.v2*_rhs, v3: self.v3*_rhs, _private: self._private }
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, _rhs: &Vector3) -> Vector3 {
        Vector3{ v1: self*_rhs.v1, v2: self*_rhs.v2, v3: self*_rhs.v3, _private: _rhs._private }
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Vector3 {
        Vector3{ v1: self.v1/_rhs, v2: self.v2/_rhs, v3: self.v3/_rhs, _private: self._private }
    }
}

impl Vector3 {
    pub fn new(v1: f64, v2: f64, v3: f64)
        -> Vector3 {
        Vector3{ v1, v2, v3, _private: () }
    }

    pub fn mag(&self) -> f64 {
        self.v1.powi(2) + self.v2.powi(2) + self.v3.powi(2) 
    }
    pub fn norm(&self) -> f64 {
        (self.v1.powi(2) + self.v2.powi(2) + self.v3.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Vector3 {
        self/self.norm()
    }
    pub fn dot(&self, rhs: Vector3) -> f64 {
        self.v1*rhs.v1 + self.v2+rhs.v2 + self.v3+rhs.v3
    }
    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Vector3{ v1: self.v2*rhs.v3-self.v3*rhs.v2,
            v2: self.v3*rhs.v1 - self.v1*rhs.v3,
            v3: self.v1*rhs.v2 - self.v2*rhs.v1,
            _private: () }
    }
}
