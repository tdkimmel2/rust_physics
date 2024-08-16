use num::complex::Complex;
use ndarray::{Array1, Array2, arr1, arr2};

pub struct Spinor { // W: struct is never constructed: `Spinor`
    pub s1: Complex<f64>,
    pub s2: Complex<f64>,
    pub spinor: Array2<Complex<f64>>,
    pub spinor_transpose: Array1<Complex<f64>>,
    _private: (),
}

impl Spinor{
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
        let norm = (norm_s1.powf(2.0)+norm_s2.powf(2.0)).sqrt();
        self.s1 = self.s1/norm;
        self.s2 = self.s2/norm;
        self.spinor = arr2(&[[self.s1],
                             [self.s2]]);
        self.spinor_transpose = arr1(&[self.s1, self.s2]);
    }

    pub fn conjugate(&self) -> Spinor {
        let spinor_conj = Spinor::new(self.s1.conj(), self.s2.conj());
        spinor_conj
    }

    pub fn construct_spinor_flag(&self, flag_length: f64, flag_width: f64) -> Vec<Vec<f64>> {
        /*
         * If wanted to use arrays
         * Didn't see benefit of doing it this way
        let unit_flag_coords = arr2(&[[0.0,0.0,0.0],
                                    [0.0,0.0,1.0],
                                    [flag_length,0.0,1.0],
                                    [flag_length,0.0,1.0-flag_width],
                                    [0.0,0.0,1.0-flag_width]]);
        */
    
        let mut unit_flag_coords:Vec<Vec<f64>> = vec![];
        unit_flag_coords.push(vec![0.0,0.0,0.0]); // Base
        unit_flag_coords.push(vec![0.0,0.0,1.0]); // Top
        unit_flag_coords.push(vec![flag_length,0.0,1.0]); // Top corner of flag
        unit_flag_coords.push(vec![flag_length,0.0,1.0-flag_width]); // bottom corner of flag
        unit_flag_coords.push(vec![0.0,0.0,1.0-flag_width]); // Bottom corner of flag, on pole
    
        unit_flag_coords
    }
}

