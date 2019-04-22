extern crate num;

use num::complex::Complex;

fn f_c(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_c_evaluates_correctly_for_complex_numbers() {
        assert_eq!(Complex::new(5.5, 13.),
            f_c(Complex::new(3., 2.), Complex::new(0.5, 1.)));
    }
}
