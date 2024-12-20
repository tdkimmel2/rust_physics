use crate::vector3::Vector3;
use crate::atmosphere::Atmosphere;

pub struct Projectile {
    pub mass: f64,
    pub drag_coefficient: f64,
    pub velocity: Vector3,
    pub spin: Vector3,
    _private: (),
}

impl Projectile {
    pub fn momentum(&self) -> Vector3 {
        self.mass*&self.velocity
    }

    pub fn kinetic_energy(&self) -> f64 {
        let momentum_mag = self.momentum().mag();
        momentum_mag/(2.*self.mass)
    }
    /*
    pub fn elastic_collision(&mut self, &mut projectile2: Projectile) {
        let veloc
    }
    */


}
