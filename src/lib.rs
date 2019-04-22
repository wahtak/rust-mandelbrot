extern crate num;

use num::complex::Complex;

fn mandelbrot_function(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

fn measure_divergence(func: &Fn(f64) -> f64, initial_value: f64, bound: f64) -> i64 {
    let mut value = initial_value;
    let mut iterations = 0;
    while value < bound {
        value = func(value);
        iterations += 1;
    }
    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandelbrot_function_evaluates_correctly_for_complex_numbers() {
        assert_eq!(Complex::new(5.5, 13.),
            mandelbrot_function(Complex::new(3., 2.), Complex::new(0.5, 1.)));
    }

    #[test]
    fn measure_divergence_counts_iterations_of_function_until_value_reaches_bound() {
        let add_one = |n| n + 1.;
        let iterations = measure_divergence(&add_one, 0., 3.);
        assert_eq!(3, iterations);
    }
}
