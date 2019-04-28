pub fn float_range(min: f64, max: f64, step: f64) -> impl std::iter::Iterator<Item = f64> {
    (((min / step) as i64)..=((max / step) as i64)).map(move |i| i as f64 * step)
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
}
