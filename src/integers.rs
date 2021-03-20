// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

fn sq(value: i32) -> i32 {
    return value * value;
}
fn round_up_div(dividend: i32, divisor: i32) -> i32 {
    return (dividend + divisor - 1) / divisor;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sq() {
        assert_eq!(9, sq(3));
        assert_eq!(9, sq(-3));
    }
    #[test]
    fn test_round_up_div() {
        assert_eq!(3, round_up_div(9, 3));
        assert_eq!(3, round_up_div(8, 3));
        assert_eq!(3, round_up_div(7, 3));
        assert_eq!(2, round_up_div(6, 3));
        assert_eq!(5, round_up_div(20, 4));
        assert_eq!(5, round_up_div(19, 4));
        assert_eq!(5, round_up_div(18, 4));
        assert_eq!(5, round_up_div(17, 4));
        assert_eq!(4, round_up_div(16, 4));
    }
}
