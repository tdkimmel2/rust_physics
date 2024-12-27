use crate::vector3::Vector3;
use crate::constants;
use crate::units;
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
        0.61078*E.powf((17.27*(self.temperature+units::KTOC-273.15))/(self.temperature+units::KTOC-35.85))
    }
    pub fn vapor_pressure(&self) -> f64 {
        self.humidity*self.saturation_pressure()
    }
    pub fn pressure(&self) -> f64 {
        1013.25*E.powf(
            (-constants::G*constants::AIR_MOL_MASS*self.elevation)/
            (constants::R*self.temperature))
    }
    pub fn dry_air_density(&self) -> f64 {
        self.pressure()*constants::AIR_MOLEC_MASS/
            (constants::KB*self.temperature)
    }
    pub fn air_density(&self) -> f64 {
        (self.dry_air_density()*constants::AIR_MOL_MASS +
         self.vapor_pressure()*constants::WATER_VAPOR_MOL_MASS)/
            (constants::R*self.temperature)
    }
}
