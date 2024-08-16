use num::complex::Complex;
use ndarray::{Array2, arr2};

pub fn get_pauli_x() -> Array2<Complex<f64>> {
    let p1 = Complex::new(0f64,0f64);
    let p2 = Complex::new(1f64,0f64);
    let p3 = Complex::new(1f64,0f64);
    let p4 = Complex::new(0f64,0f64);
    let pauli = arr2(&[[p1,p2],
                       [p3,p4]]);
    //let pauli:Vec<Vec<Complex<f64>>> = vec![vec![p1,p2],vec![p3,p4]];

    pauli
}

pub fn get_pauli_y() -> Array2<Complex<f64>> {
    let p1 = Complex::new(0f64,0f64);
    let p2 = Complex::new(0f64,-1f64);
    let p3 = Complex::new(0f64,1f64);
    let p4 = Complex::new(0f64,0f64);
    let pauli = arr2(&[[p1,p2],
                       [p3,p4]]);
    //let pauli:Vec<Vec<Complex<f64>>> = vec![vec![p1,p2],vec![p3,p4]];

    pauli
}

pub fn get_pauli_z() -> Array2<Complex<f64>> {
    let p1 = Complex::new(1f64,0f64);
    let p2 = Complex::new(0f64,0f64);
    let p3 = Complex::new(0f64,0f64);
    let p4 = Complex::new(-1f64,0f64);
    let pauli = arr2(&[[p1,p2],
                       [p3,p4]]);
    //let pauli:Vec<Vec<Complex<f64>>> = vec![vec![p1,p2],vec![p3,p4]];

    pauli
}
