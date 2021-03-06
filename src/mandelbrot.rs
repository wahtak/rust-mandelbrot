extern crate num;

use num::complex::Complex64;

pub trait Bounded {
    fn out_of_bound(&self, bound: f64) -> bool;
}

impl Bounded for Complex64 {
    fn out_of_bound(&self, bound: f64) -> bool {
        (self.re).abs() > bound || (self.im).abs() > bound
    }
}

impl Bounded for f64 {
    fn out_of_bound(&self, bound: f64) -> bool {
        (*self).abs() > bound
    }
}

pub fn mandelbrot_function(c: Complex64) -> Box<Fn(Complex64) -> Complex64> {
    Box::new(move |z| z * z + c)
}

pub fn measure_divergence<T: Bounded + num::Zero>(
    function: &Fn(T) -> T,
    bound: f64,
    max_iterations: i64,
) -> Option<i64> {
    let mut value: T = num::zero();
    let mut iterations = 0;
    loop {
        if value.out_of_bound(bound) {
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
        let z = Complex64 { re: 3.0, im: 2.0 };
        let c = Complex64 { re: 0.5, im: 1.0 };

        assert_eq!(Complex64 { re: 5.5, im: 13. }, mandelbrot_function(c)(z));
    }

    #[test]
    fn measure_divergence_returns_number_of_iterations_after_which_value_is_out_of_bound() {
        let add_constant = |z: f64| z + 1.0;
        let bound = 3.5;

        let iterations = measure_divergence(&add_constant, bound, 10);

        assert_eq!(4, iterations.unwrap());
    }

    #[test]
    fn measure_divergence_returns_none_when_number_of_iterations_exceeds_maximum() {
        let negate = |z: f64| z * -1.;
        let max_iterations = 10;

        let iterations = measure_divergence(&negate, 3.5, max_iterations);

        assert!(iterations.is_none());
    }

    #[test]
    fn mandelbrot_function_does_not_diverge_for_some_constant() {
        let c = Complex64 { re: 0.1, im: 0.1 };

        let iterations = measure_divergence(&*mandelbrot_function(c), 1.0, 10);

        assert!(iterations.is_none());
    }

    #[test]
    fn mandelbrot_function_diverges_for_some_constant() {
        let c = Complex64 { re: 0.5, im: 0.5 };

        let iterations = measure_divergence(&*mandelbrot_function(c), 1.0, 10);

        assert!(iterations.is_some());
    }
}
