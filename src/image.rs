// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

use core::panic;

pub struct FingerprintImageOptions {
    dpi: f64,
}

impl Default for FingerprintImageOptions {
    fn default() -> Self {
        FingerprintImageOptions {
            dpi: 500.0,
        }
    }
}

impl FingerprintImageOptions {
    pub fn dpi(&mut self, dpi: f64) -> &mut Self {
        if dpi < 20.0 || dpi > 20_000.0 {
            panic!("DPI is non-positive, impossibly low, or impossibly high.");
        }
        self.dpi = dpi;
        self
    }
}