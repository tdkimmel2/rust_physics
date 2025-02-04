use crate::vector3::Vector3;
use crate::constants;
use crate::si_units as units;
use std::f64::consts::E;

#[derive(Copy, Clone)]
pub struct Atmosphere {
    pub temperature: f64,
    pub humidity: f64,
    pub elevation: f64,
    pub wind: Vector3,
    _private: (),
}

impl Atmosphere {
    pub fn new(temperature: f64, humidity: f64,
        elevation: f64, wind: Vector3)
        -> Atmosphere {
        Atmosphere{ temperature, humidity,
            elevation, wind, _private: () }
    }

    pub fn saturation_pressure(&self) -> f64 {
        // Tetens Equation, gives pressure in kPa
        // https://en.wikipedia.org/wiki/Tetens_equation
        // Magic coefficients
        let coef1 = 0.61078;
        let coef2 = 17.27;
        let exp_num = coef2 * self.temperature;
        if self.temperature * units::KTOC < 0. {
            let low_temp_shift = 265.5;
            let exp_denom = self.temperature + units::KTOC + low_temp_shift;
            coef1 * E.powf(exp_num / exp_denom) * units::PA / units::KPA
        }
        else {
            let high_temp_shift = 237.3;
            let exp_denom = self.temperature + units::KTOC + high_temp_shift;
            coef1 * E.powf(exp_num / exp_denom) * units::PA / units::KPA
        }
    }
    pub fn vapor_pressure(&self) -> f64 {
        self.humidity * self.saturation_pressure()
    }
    pub fn pressure(&self) -> f64 {
        let exp_num = -constants::G * constants::AIR_MOL_MASS * self.elevation;
        let exp_denom = constants::R * self.temperature;
        E.powf(exp_num / exp_denom) * constants::PRESSURE_SEA_LEVEL
    }
    pub fn dry_air_density(&self) -> f64 {
        self.pressure() * constants::AIR_MOLEC_MASS /
            (constants::KB * self.temperature)
    }
    pub fn air_density(&self) -> f64 {
        let num = self.dry_air_density() * constants::AIR_MOL_MASS +
            self.vapor_pressure() * constants::WATER_VAPOR_MOL_MASS;
        let denom = constants::R * self.temperature;
        num / denom
    }
}
