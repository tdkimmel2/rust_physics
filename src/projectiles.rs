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
    pub magnus_coefficient: f64,
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
            magnus_coefficient: 0.,
            position: Vector3::new(0., 0., 0.),
            spin: Vector3::new(0., 0., 0.),
            velocity: Vector3::new(0., 0., 0.),
            theta: 0.,
            speed: 0.,
            phi: 0.,
            _private: (),
        }
    }

    // Setters

    // Set speed and angles, updates velocity accordingly
    pub fn set_speed_theta
        (&mut self, speed: f64, theta: f64) {
        let x = speed * tan(theta);
        let y = 0.;
        let z = x * cos(theta);

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
        let forward_mag = speed * cos(theta);
        let x = forward_mag * cos(phi);
        let y = forward_mag * sin(phi);
        let z = speed * sin(theta);

        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
        self.theta = theta;
        self.speed = speed;
        self.phi = phi;
    }

    // Set velocity, updates speed and angles accordingly
    pub fn set_velocity_components
        (&mut self, vx: f64, vy: f64, vz: f64) {
        let speed = self.velocity.mag();
        let xy_mag = (vx.powi(2) + vy.powi(2)).sqrt();
        let phi = acos(vx / xy_mag);

        self.velocity.x = vx;
        self.velocity.y = vy;
        self.velocity.z = vz;
        self.speed = speed;
        self.theta = atan(vz / xy_mag);
        self.phi = phi;
    }

    pub fn set_velocity
        (&mut self, velocity: Vector3) {
        let vx = velocity.x;
        let vy = velocity.y;
        let vz = velocity.z;
        let xy_mag = (vx.powi(2) + vy.powi(2)).sqrt();
        let speed = self.velocity.mag();
        let phi = acos(vx / xy_mag);

        self.velocity = velocity;
        self.speed = speed;
        self.theta = atan(vz / xy_mag);
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

    pub fn kinetic_energy(&self) -> f64 {
        let momentum_mag2 = self.momentum().mag2();
        momentum_mag2 / (2. * self.mass)
    }

    pub fn momentum(&self) -> Vector3 {
        self.mass * &self.velocity
    }


    /*******************
    ********************
    Quantities in Vacuum
    ********************
    *******************/
    pub fn apex_vacuum_time(&self) -> f64 {
        let g = -constants::G;
        let t = -self.velocity.z / g;
        t
    }

    pub fn apex_vacuum(&self) -> f64 {
        let g = -constants::G;
        let t = self.apex_vacuum_time();
        self.position.z + self.velocity.z * t + g * t.powi(2) / 2.
    }

    pub fn range_vacuum_time(
        &self, end_height: f64) -> f64 {
    
        let deltaz = self.position.z + end_height;
        let g = -constants::G;

        // Time
        (-self.velocity.z - (self.velocity.z.powi(2) - 2. * deltaz * g).sqrt()) / g
    }

    pub fn range_vacuum(
        &self, end_height: f64) -> f64 {

        let t = self.range_vacuum_time(end_height);
    
        let final_pos_x = self.position.x + self.velocity.x * t;
        let final_pos_y = self.position.y + self.velocity.y * t;
        let final_pos_z = self.position.z + self.velocity.z * t;
        let final_position = Vector3::new(final_pos_x, final_pos_y, final_pos_z);
    
        let deltax = final_position.x - self.position.x;
        let deltay = final_position.y - self.position.y;
        (deltax.powi(2) + deltay.powi(2)).sqrt()
    }

    pub fn trajectory_vaccum(
        &mut self, end_height: f64, max_time: f64) -> Vec<Vector3> {

        let mut falling: bool = false;
        let mut traj: Vec<Vector3> = Vec::new();
        traj.push(self.position);
        let accel= Vector3::new(0., 0., -constants::G);

        let mut t: f64 = 0.;
        let t_step = 0.1;
        while (self.position.z > end_height && falling) || t < max_time {
            let new_position = self.position + self.velocity * t_step;
            falling = self.position.z > new_position.z;
            self.position =  new_position;
            self.set_velocity(self.velocity + accel * t_step);
            traj.push(self.position);
            t = t + t_step;
        }

        traj
    }


    /******************
    *******************
    With Air Resistance
    *******************
    ******************/
    pub fn air_resistance(&self, atm: &Atmosphere, speed: f64) -> f64 {
        let surface_area: f64 = 4. * consts::PI * self.radius.powi(2);
        self.drag_coefficient * atm.air_density()
            * surface_area * speed.powi(2) / 2.
    }

    pub fn force(&self, atm: &Atmosphere) -> Vector3 {
        let air_res_x = self.air_resistance(atm, self.velocity.x + atm.wind.x);
        let air_res_y = self.air_resistance(atm, self.velocity.y + atm.wind.y);
        let air_res_z = self.air_resistance(atm, self.velocity.z + atm.wind.z);

        let magnus: Vector3 = Vector3::cross_prod(&self.spin, &self.velocity) *
                                self.magnus_coefficient;

        let force_x = magnus.x - air_res_x;
        let force_y = magnus.y - air_res_y;
        let force_z = magnus.z - air_res_z - constants::G;

        Vector3::new(force_x, force_y, force_z)
    }

    pub fn acceleration(&self, atm: &Atmosphere) -> Vector3 {
        &self.force(atm) / self.mass
    }

    pub fn trajectory(
        &mut self, atm: &Atmosphere, end_height: f64, max_time: f64) -> Vec<Vector3> {

        let mut falling: bool = false;
        let mut traj: Vec<Vector3> = Vec::new();
        traj.push(self.position);

        let mut t: f64 = 0.;
        let t_step = 0.1;
        while (self.position.z > end_height && falling) || t < max_time {
            let new_position = self.position + self.velocity * t_step;
            falling = self.position.z > new_position.z;
            self.position = new_position;
            let accel = self.acceleration(atm);
            self.set_velocity(self.velocity + accel * t_step);
            traj.push(self.position);
            t = t + t_step;
        }

        traj
    }
}

