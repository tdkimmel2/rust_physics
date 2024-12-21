use std::ops::{Add, Sub, Mul, Div};

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    _private: (),
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3{ x: self.x + _rhs.x,
        y: self.y + _rhs.y,
        z: self.z + _rhs.z,
        _private: () }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3{ x: self.x - _rhs.x,
        y: self.y - _rhs.y,
        z: self.z - _rhs.z,
        _private: () }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3{ x: self.x*_rhs, y: self.y*_rhs, z: self.z*_rhs, _private: self._private }
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, _rhs: &Vector3) -> Vector3 {
        Vector3{ x: self*_rhs.x, y: self*_rhs.y, z: self*_rhs.z, _private: _rhs._private }
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Vector3 {
        Vector3{ x: self.x/_rhs,
        y: self.y/_rhs,
        z: self.z/_rhs,
        _private: self._private }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64)
        -> Vector3 {
        Vector3{ x, y, z, _private: () }
    }

    pub fn mag2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2) 
    }
    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }
    pub fn normalize(&self) -> Vector3 {
        self/self.mag()
    }

    pub fn dot(&self, rhs: Vector3) -> f64 {
        self.x*rhs.x + self.y+rhs.y + self.z+rhs.z
    }
    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Vector3{ x: self.y*rhs.z-self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
            _private: () }
    }
}
