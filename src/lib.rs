// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

mod image;
mod number;
mod point;

pub(crate) use point::DoublePoint;
pub(crate) use point::IntPoint;
pub(crate) use point::Point;

pub use image::FingerprintImageOptions;

#[cfg(test)]
mod test_utils {
    pub fn assert_approx_eq(expected: f64, actual: f64, tolerance: f64) {
        assert!((actual - expected).abs() <= tolerance);
    }
}
