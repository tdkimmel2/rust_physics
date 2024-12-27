use core::f64::consts;
use libm::{cos,acos,sin,tan,atan};

use crate::vector3::Vector3;
use crate::constants;
use crate::atmosphere::Atmosphere;

#[derive(Copy, Clone)]
pub struct Projectile {
    pub mass: f64,
    pub radius: f64,
    pub drag_coefficient: f64,
    pub position: Vector3,
    pub spin: Vector3,
    velocity: Vector3,
    speed: f64,
    theta: f64,
    phi: f64,
    _private: (),
}

impl Projectile {
    pub fn new() -> Projectile {
        Projectile{
            mass: 0.,
            radius: 0.,
            drag_coefficient: 0.,
            position: Vector3::new(0.,0.,0.),
            spin: Vector3::new(0.,0.,0.),
            velocity: Vector3::new(0.,0.,0.),
            theta: 0.,
            speed: 0.,
            phi: 0.,
            _private: (),
        }
    }

    // Setters

    // Set speed, theta with corresponding velocity
    pub fn set_speed_theta
        (&mut self, speed: f64, theta: f64) {
        let x = speed*tan(theta);
        let y = 0.;
        let z = x*cos(theta);

        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
        self.theta = theta;
        self.speed = speed;
        self.phi = 0.;
    }
    pub fn set_speed_theta_phi
        (&mut self, speed: f64, theta: f64,
         phi: f64) {
        let forward_mag = speed*cos(theta);
        let x = forward_mag*cos(phi);
        let y = forward_mag*sin(phi);
        let z = speed*sin(theta);

        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
        self.theta = theta;
        self.speed = speed;
        self.phi = phi;
    }
    // Set velocity and corresponding thetas
    pub fn set_velocity_components
        (&mut self, vx: f64, vy: f64, vz: f64) {
        let speed = self.velocity.mag();
        let xy_mag = (vx.powi(2) + vy.powi(2)).sqrt();
        let phi = acos(vx/xy_mag);

        self.velocity.x = vx;
        self.velocity.y = vy;
        self.velocity.z = vz;
        self.speed = speed;
        self.theta = atan(vz/xy_mag);
        self.phi = phi;
    }
    pub fn set_velocity
        (&mut self, velocity: Vector3) {
        let vx = velocity.x;
        let vy = velocity.y;
        let vz = velocity.z;
        let xy_mag = (vx.powi(2)+vy.powi(2)).sqrt();
        let speed = self.velocity.mag();
        let phi = acos(vx/xy_mag);

        self.velocity = velocity;
        self.speed = speed;
        self.theta = atan(vz/xy_mag);
        self.phi = phi;
    }

    // Getters
    pub fn get_theta(&self) -> f64 {
        self.theta
    }
    pub fn get_phi(&self) -> f64 {
        self.phi
    }
    pub fn get_speed(&self) -> f64 {
        self.speed
    }
    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn momentum(&self) -> Vector3 {
        self.mass*&self.velocity
    }

    pub fn kinetic_energy(&self) -> f64 {
        let momentum_mag = self.momentum().mag();
        momentum_mag/(2.*self.mass)
    }


    // Quantities in Vacuum
    pub fn apex_vacuum_time(&self) -> f64 {
        let g = -constants::G;
        let t = -self.velocity.z/g;
        t
    }
    pub fn apex_vacuum(&self) -> f64 {
        let g = -constants::G;
        let t = self.apex_vacuum_time();
        self.position.z + self.velocity.z*t + g*t.powi(2)/2.
    }
    pub fn range_vacuum_time(
        &self, elevation: f64) -> f64 {
    
        let deltaz = elevation - self.position.z;
        let g = -constants::G;
        let t = (-self.velocity.z-(self.velocity.z.powi(2)-2.*deltaz*g).sqrt())/g;
        t 
    }
    pub fn range_vacuum(
        &self, elevation: f64) -> f64 {

        /*
        let deltaz = elevation - self.position.z;
        let g = -constants::G;
        let t = (-self.velocity.z-(self.velocity.z.powi(2)-2.*deltaz*g).sqrt())/g;
        */
        let t = self.range_vacuum_time(elevation);
    
        let mut final_position = Vector3::new(0.,0.,0.);
        final_position.x = self.position.x + self.velocity.x*t;
        final_position.y = self.position.y + self.velocity.y*t;
        final_position.z = self.position.z + self.velocity.z*t;
    
        let deltax = final_position.x - self.position.x;
        let deltay = final_position.y - self.position.y;
        (deltax.powi(2) + deltay.powi(2)).sqrt()
    }

    // With Air Resistance
    pub fn air_resistance(&self, atmosphere: Atmosphere) -> f64 {
        let surface_area:f64 = 4.*consts::PI*self.radius.powi(2);
        self.drag_coefficient*atmosphere.air_density()
            *surface_area*self.velocity.mag2()/2.
    }
    /*
    pub fn acceleration(&self, atm: Atmosphere) -> Vector3 {
        let air_density = atm.air_density();
    }
    */
}

