extern crate num;

use num::complex::Complex64;

fn mandelbrot_function(z: Complex64, c: Complex64) -> Complex64 {
    z * z + c
}

fn measure_divergence(function: &Fn(i64) -> i64, initial_value: i64, bound: i64, max_iterations: i64) -> Option<i64>
{
    let mut value = initial_value;
    let mut iterations = 0;
    loop {
        if num::abs(value) > bound {
            return Some(iterations);
        }
        if iterations == max_iterations {
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
        let z = Complex64{re: 3., im: 2.};
        let c = Complex64{re: 0.5, im: 1.};

        assert_eq!(Complex64{re: 5.5, im: 13.}, mandelbrot_function(z, c));
    }

    #[test]
    fn measure_divergence_counts_iterations_of_closure_until_value_reaches_bound() {
        let c = 1;
        let add_c = |i| i + c;
        let initial_value = 0;
        let bound = 3;

        let iterations = measure_divergence(&add_c, initial_value, bound, 5);

        assert_eq!(4, iterations.unwrap());
    }

    #[test]
    fn measure_divergence_outputs_none_when_number_of_iterations_exceeds_maximum() {
        let negate = |i| i * -1;
        let max_iterations = 5;

        let iterations = measure_divergence(&negate, 0, 3, max_iterations);

        assert!(iterations.is_none());
    }
}
