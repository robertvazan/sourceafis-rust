// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

mod double_point;
mod int_point;
mod integers;

pub(crate) use double_point::DoublePoint;

#[cfg(test)]
mod test_utils {
    pub fn assert_approx_eq(expected: f64, actual: f64, tolerance: f64) {
        assert!((actual - expected).abs() <= tolerance);
    }
}
