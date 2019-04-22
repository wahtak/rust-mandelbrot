extern crate num;

use num::complex::Complex;

fn mandelbrot_function(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

fn measure_divergence(function: &Fn(i64) -> i64, initial_value: i64, bound: i64, maximum_iterations: i64) -> Option<i64>
{
    let mut value = initial_value;
    let mut iterations = 0;
    loop {
        if value > bound {
            return Some(iterations);
        }
        if iterations == maximum_iterations {
            return None;
        }
        value = function(value);
        iterations += 1;
    }

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
    fn measure_divergence_counts_iterations_of_closure_until_value_reaches_bound() {
        let c = 1;
        let add_c = |i| i + c;
        let iterations = measure_divergence(&add_c, 0, 3, 5);
        assert_eq!(4, iterations.unwrap());
    }

    #[test]
    fn measure_divergence_outputs_none_when_number_of_iterations_exceeds_maximum() {
        let negate = |i| i * -1;
        let iterations = measure_divergence(&negate, 0, 3, 5);
        assert!(iterations.is_none());
    }
}
