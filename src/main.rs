mod draw;
mod mandelbrot;

use num::complex::Complex64;

fn main() {
    for im in draw::float_range(-1.0, 1.0, 0.05) {
        for re in draw::float_range(-2.0, 1.0, 0.025) {
            let c = Complex64 { re: re, im: im };
            let divergence =
                mandelbrot::measure_divergence(&*mandelbrot::mandelbrot_function(c), 3.0, 100);
            print!(
                "{}",
                match divergence {
                    None => "*",
                    Some(_) => " ",
                }
            );
        }
        println!();
    }
}
