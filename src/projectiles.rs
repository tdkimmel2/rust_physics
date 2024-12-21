use crate::vector3::Vector3;
use crate::constants;
use crate::atmosphere::Atmosphere;
use f64::consts;

pub struct Projectile {
    pub mass: f64,
    pub radius: f64,
    pub drag_coefficient: f64,
    pub position: Vector3,
    pub velocity: Vector3,
    pub spin: Vector3,
    _private: (),
}

impl Projectile {
    pub fn new() -> Projectile {
        Projectile{
            mass: 0.,
            radius: 0.,
            drag_coefficient: 0.,
            position: Vector3::new(0.,0.,0.),
            velocity: Vector3::new(0.,0.,0.),
            spin: Vector3::new(0.,0.,0.),
            _private: (),
        }
    }

    pub fn momentum(&self) -> Vector3 {
        self.mass*&self.velocity
    }

    pub fn kinetic_energy(&self) -> f64 {
        let momentum_mag = self.momentum().mag();
        momentum_mag/(2.*self.mass)
    }

    pub fn air_resistance(&self, atmosphere: Atmosphere) -> f64 {
        let surface_area:f64 = 4.*consts::PI*self.radius.powi(2);
        self.drag_coefficient*atmosphere.air_density()
            *surface_area*self.velocity.mag2()/2.
    }

    pub fn apex_vacuum(&self) -> f64 {
        let g = -constants::G;
        let t = -self.velocity.z/g;
        self.position.z + self.velocity.z*t + g*t.powi(2)/2.
    }
    pub fn range_vacuum(
        &self, elevation: f64) -> f64 {
    
        let deltaz = elevation - self.position.z;
        let g = -constants::G;
        let t = (-self.velocity.z-(self.velocity.z.powi(2)-2.*deltaz*g).sqrt())/g;
    
        let mut final_position = Vector3::new(0.,0.,0.);
        final_position.x = self.position.x + self.velocity.x*t;
        final_position.y = self.position.y + self.velocity.y*t;
        final_position.z = self.position.z + self.velocity.z*t;
    
        let deltax = final_position.x - self.position.x;
        let deltay = final_position.y - self.position.y;
        (deltax.powi(2) + deltay.powi(2)).sqrt()
    }
}

