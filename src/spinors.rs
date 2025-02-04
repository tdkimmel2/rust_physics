use std::f64::consts::PI;
use num::complex::Complex;
use ndarray::{Array1, Array2, arr1, arr2};
use libm::{cos, acos, sin, atan2};

pub struct Spinor { // W: struct is never constructed: `Spinor`
    pub s1: Complex<f64>,
    pub s2: Complex<f64>,
    pub spinor: Array2<Complex<f64>>,
    pub spinor_transpose: Array1<Complex<f64>>,
    _private: (),
}

impl Spinor {
    pub fn new(s1: Complex<f64>, s2: Complex<f64>) -> Spinor {
        if s1.norm()==0. && s2.norm()==0. {
            println!("WARNING: Initializing spinor with 0 size");
        }
        let spinor = arr2(&[[s1],
                            [s2]]);
        let spinor_transpose = arr1(&[s1, s2]);
        Spinor { s1, s2, spinor, spinor_transpose, _private: () }
    }

    pub fn print(&mut self) {
        println!("[{:.4}\n {:.4}]", self.s1, self.s2);
        println!("[{:?}", self.spinor);
    }

    pub fn normalize(&mut self) {
        let norm_s1 = self.s1.norm();
        let norm_s2 = self.s2.norm();
        let norm = (norm_s1.powf(2.0) + norm_s2.powf(2.0)).sqrt();
        self.s1 = self.s1 / norm;
        self.s2 = self.s2 / norm;
        self.spinor = arr2(&[[self.s1],
                             [self.s2]]);
        self.spinor_transpose = arr1(&[self.s1, self.s2]);
    }

    pub fn conj(&self) -> Spinor {
        let spinor_conj = Spinor::new(self.s1.conj(), self.s2.conj());
        spinor_conj
    }

    pub fn rotate_phase(&mut self, phase:Complex<f64>) {
       let exp_phase = Complex::exp(phase);
       self.s1 = self.s1 * exp_phase;
       self.s2 = self.s2 * exp_phase;
    }

    pub fn get_phi(&self) -> f64 {
        let r_y = (Complex::new(0., 1.) *
                    (self.s1 * self.s2.conj() - self.s2 * self.s1.conj())).re;
        let r_z = self.s1.norm_sqr() - self.s2.norm_sqr();
        let phi = atan2(r_y, r_z);

        phi
    }

    pub fn get_alpha(&self) -> f64 {
        let phi = self.get_phi();
        let mut alpha = -2.0 * (self.s1 - Complex::new(phi, 0.)).arg();
        alpha = alpha % (2. * PI);
      	if alpha.abs() < 0.00001 { alpha = 0.; }
      	if alpha > 2. * PI - 0.0001 { alpha = 0.; }

        alpha
    }

    pub fn get_theta(&self) ->f64 {
        let r = self.s1.norm_sqr() + self.s2.norm_sqr();
        let r_z = self.s1.norm_sqr() - self.s2.norm_sqr();
        let theta = acos(r_z / r);

        theta
    }

    pub fn construct_spinor_flag(&mut self, flag_length: f64, flag_width: f64) -> Vec<(f64, f64, f64)> {
        let r = self.s1.norm_sqr() + self.s2.norm_sqr();
		let r_x = (self.s1 * self.s2.conj() + self.s2 * self.s1.conj()).re;
      	let r_y = (Complex::new(0.,1.) *
                    (self.s1 * self.s2.conj() - self.s2 * self.s1.conj())).re;
      	let r_z = self.s1.norm_sqr() - self.s2.norm_sqr();

        // Calculate rather than get_phi() since we already have r* vals
		let phi = atan2(r_y, r_z); 
        // Calculate rather than get_theta() since we already have r* vals
    	//let theta = acos(r_z/r);

      	if self.s1.norm() < 0.000001 { self.s1 = Complex::new(0.000001, 0.); }
      	let alpha = self.get_alpha();
      	if self.s1.norm() < 0.00001 { self.s1 = Complex::new(0., 0.); }

        let perp_vec = arr1(&[r_z * cos(phi),
                              r_z*sin(phi),
                              -(r_x * r_x+r_y * r_y).sqrt()]);
        let unit_fvec = arr1(&[r_x, r_y, r_z]) / r;

        // Dirty way to outer product
        // https://github.com/rust-ndarray/ndarray-linalg/issues/43
        let unit_fvec_row = arr2(&[[r_x, r_y, r_z]]) / r;
        let unit_fvec_col = arr2(&[[r_x], [r_y], [r_z]]) / r;
        let outer_mat = unit_fvec_col.dot(&unit_fvec_row);

        // Identity matrix
        let i3 = arr2(&[[1., 0., 0.],
                        [0., 1., 0.],
                        [0., 0., 1.]]);
        // Cross matrix
        let cross_mat = arr2(&[[0., -unit_fvec[2], unit_fvec[1]],
                               [unit_fvec[2], 0., -unit_fvec[0]],
                               [-unit_fvec[1], unit_fvec[0], 0.]]);
        // Rodrigues formula
        let rot_mat = cos(alpha) * &i3 + (1. - cos(alpha)) * &outer_mat + sin(alpha) * &cross_mat;
        let flag_vec = flag_length * rot_mat.dot(&perp_vec);
   
        // Create output of vector of 3D coordinates for plotting
        let mut unit_flag_coords:Vec<(f64, f64, f64)> = vec![];
        unit_flag_coords.push((0., 0., 0.)); // Base
        unit_flag_coords.push((r_x, r_y, r_z)); // Top
        unit_flag_coords.push((r_x + flag_vec[0],
                               r_y + flag_vec[1],
                               r_z + flag_vec[2])); // Top corner of flag
        unit_flag_coords.push(((1. - flag_width) * r_x + flag_vec[0],
                               (1. - flag_width) * r_y + flag_vec[1],
                               (1. - flag_width) * r_z + flag_vec[2])); // bottom corner of flag
        unit_flag_coords.push(((1. - flag_width) * r_x,
                               (1. - flag_width) * r_y,
                               (1. - flag_width) * r_z)); // Bottom corner of flag, on pole
   
        unit_flag_coords
    }
}
