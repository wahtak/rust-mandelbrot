pub fn f_c(z: f64, c: f64) -> f64 {
    z * z + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_c_evaulates_correctly_for_real_numbers() {
        assert_eq!(0., f_c(0., 0.));
        assert_eq!(5., f_c(2., 1.));
    }
}
