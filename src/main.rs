mod lib;

fn main() {
	for coords in lib::complex_grid(-1.0, 1.0, 0.1, -1.0, 1.0, 0.1) {
		println!("{:?}", coords );
	}
}
