extern crate image;

pub fn float_range(min: f64, max: f64, step: f64) -> impl std::iter::Iterator<Item = f64> {
    (((min / step) as i64)..=((max / step) as i64)).map(move |i| i as f64 * step)
}

pub fn draw(color_function : &Fn(u32, u32) -> image::Rgb<u8>) -> image::RgbImage {
	image::RgbImage::from_fn(256, 256, color_function)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_range_iterates_over_range() {
        let mut iterator = float_range(-1.0, 1.0, 0.4);

        let is_near = |a: f64, b: f64| (a - b).abs() < std::f64::EPSILON;
        assert!(is_near(iterator.next().unwrap(), -0.8));
        assert!(is_near(iterator.next().unwrap(), -0.4));
        assert!(is_near(iterator.next().unwrap(), 0.0));
        assert!(is_near(iterator.next().unwrap(), 0.4));
        assert!(is_near(iterator.next().unwrap(), 0.8));
        assert!(iterator.next().is_none());
    }

    #[test]
    fn draw_creates_rbg_image_from_color_function() {
    	let color_function = |x : u32, y: u32| return image::Rgb([(x % 256) as u8, (y % 256) as u8, 0]);
    	let img : image::RgbImage = draw(&color_function);
    	assert!(*img.get_pixel(0, 0) == color_function(0, 0));
    	assert!(*img.get_pixel(255, 255) == color_function(255, 255));
    }
}
