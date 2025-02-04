use core::f64::consts;

// SI Units are Default

// Length
pub const M:f64 = 1.;
pub const KM:f64 = 1e3 * M;
pub const CM:f64 = 100. * M;
pub const MM:f64 = 1e-3 * M;
pub const UM:f64 = 1e-6 * M;
pub const NM:f64 = 1e-9 * M;

pub const MILES:f64 = 1_609.344 * M;

// Time
pub const S:f64 = 1.;
pub const MS:f64 = 1e-3 * S;
pub const US:f64 = 1e-6 * S;
pub const NS:f64 = 1e-9 * S;

pub const H:f64 = 3_600. * S;
pub const MIN:f64 = 60. * S;

// Mass
pub const KG:f64 = 1.;
pub const G:f64 = 1e3 * KG;
pub const MG:f64 = 1e6 * KG;

pub const LBS:f64 = 0.453_592_4 * KG;

// Energy
pub const J:f64 = KG * M * M / (S * S);

// Angles
pub const RAD:f64 = 1.;
pub const DEGREES:f64 = 2. * consts::PI * RAD / 360.;
pub const ARCMIN:f64 = DEGREES / 60.;
pub const ARCSEC:f64 = DEGREES / 3_600.;

// Pressure
pub const PA:f64 = 1.;
pub const KPA:f64 = 1e-3 * PA;

// Temperature
pub const K:f64 = 1.;
pub const KTOC:f64 = 273.15;

// Amount
pub const MOL:f64 = 1.;


/// Possibly convienient conversion functions
/// May remove if too redundant
pub fn miles_to_km(miles: f64) -> f64 {
    miles * MILES / KM
}
pub fn miles_to_m(miles: f64) -> f64 {
    miles * MILES / M
}

pub fn lbs_to_kg(lbs: f64) -> f64 {
    lbs * LBS / KG
}

pub fn mph_to_mps(mph: f64) -> f64 {
     mph * MILES / H
}

// Angles
pub fn rad_to_degrees(rad: f64) -> f64 {
    rad * 360. / (2. * consts::PI)
}
pub fn degrees_to_rad(degrees: f64) -> f64 {
    degrees * 2. * consts::PI / 360.
}

// Temperature
pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin + 272.15
}
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius - 272.15
}
pub fn celsius_to_farenheit(celsius: f64) -> f64 {
    celsius * 9. / 5. + 32.
}
pub fn farenheit_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.) * 5. / 9.
}
