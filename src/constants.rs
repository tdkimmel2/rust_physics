use crate::si_units as units;

// Speed of light
pub const C:f64 = 299_792_458.*units::M/units::S;

// Gravitational constant on Earth at sea level (m/(s^2))
pub const G:f64 = 9.80665*units::M/pow(units::S,2.);

// Boltzmann constant (J/K)
pub const KB:f64 = 1.380_649e-23*units::J/units::K;
// Avogadro number
pub const NA:f64 = 6.022_140_76.e23/units::MOL;

// Sea level standard atmospheric pressure
pub const PRESSURE_SEA_LEVEL:f64 = 101_325.*units::PA;

// Molecular mass for air (kg)
pub const AIR_MOLEC_MASS:f64 = 4.81e-26*units::KG;
// Molar mass for air (kg/mol)
pub const AIR_MOL_MASS:f64 = 0.028_964_4*units::KG/units::MOL;
// Molar mass for water vapor (kg)
pub const WATER_VAPOR_MOL_MASS:f64 = 4.81e-26*units::KG;

// Gas constant (J/(mol*K))
pub const R:f64 = 8.314_462_618_153_24*units::J/(units::MOL*units::K);
// Specific Gas constant for dry air  (J/(mol*K))
pub const R_SPEC_DRY_AIR:f64 = 287.050_067_6*units::J/(units::MOL*units::K);
