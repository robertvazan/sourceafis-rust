// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

#[cfg(test)]
use crate::test_utils::*;

#[derive(Debug, Copy, Clone)]
pub struct DoublePoint {
    x: f64,
    y: f64,
}

impl DoublePoint {
    pub const fn new(x: f64, y: f64) -> Self {
        DoublePoint { x, y }
    }
}

#[cfg(test)]
pub fn assert_eq(expected: DoublePoint, actual: DoublePoint, tolerance: f64) {
    assert_approx_eq(expected.x, actual.x, tolerance);
    assert_approx_eq(expected.y, actual.y, tolerance);
}
