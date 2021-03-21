// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

use super::*;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntPoint {
    x: i32,
    y: i32,
}

pub struct IntPointIterator {
    range: IntPoint,
    at: IntPoint,
}

impl Add for IntPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for IntPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for IntPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Ord for IntPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for IntPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Iterator for IntPointIterator {
    type Item = IntPoint;

    fn next(&mut self) -> Option<IntPoint> {
        if self.at.y < self.range.y && self.at.x < self.range.x {
            let result = self.at;
            self.at.x = self.at.x + 1;
            if self.at.x >= self.range.x {
                self.at.y = self.at.y + 1;
                self.at.x = 0;
            }
            Some(result)
        } else {
            None
        }
    }
}

impl IntoIterator for IntPoint {
    type Item = IntPoint;
    type IntoIter = IntPointIterator;

    fn into_iter(self) -> Self::IntoIter {
        IntPointIterator {
            range: self,
            at: IntPoint::ZERO,
        }
    }
}

impl IntPoint {
    const ZERO: Self = Self::new(0, 0);
    const EDGE_NEIGHBORS: [IntPoint; 4] = [
        Self::new(0, -1),
        Self::new(-1, 0),
        Self::new(1, 0),
        Self::new(0, 1),
    ];
    const CORNER_NEIGHBORS: [IntPoint; 8] = [
        Self::new(-1, -1),
        Self::new(0, -1),
        Self::new(1, -1),
        Self::new(-1, 0),
        Self::new(1, 0),
        Self::new(-1, 1),
        Self::new(0, 1),
        Self::new(1, 1),
    ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn area(self) -> i32 {
        self.x * self.y
    }
    pub fn length_sq(self) -> i32 {
        integers::sq(self.x) + integers::sq(self.y)
    }
    pub fn contains(self, other: Self) -> bool {
        other.x >= 0 && other.y >= 0 && other.x < self.x && other.y < self.y
    }
    pub fn to_double_point(self) -> DoublePoint {
        DoublePoint::new(self.x as f64, self.y as f64)
    }
    fn line_to(self, to: Self) -> Vec<Self> {
        let relative = to - self;
        if relative.x.abs() >= relative.y.abs() {
            let mut result = vec![Self::ZERO; relative.x.abs() as usize + 1];
            let slope = relative.y as f64 / (relative.x as f64);
            if relative.x > 0 {
                for i in 0..=relative.x {
                    result[i as usize] =
                        Self::new(self.x + i, self.y + (i as f64 * slope).round() as i32);
                }
            } else if relative.x < 0 {
                for i in 0..=-relative.x {
                    result[i as usize] =
                        Self::new(self.x - i, self.y - (i as f64 * slope).round() as i32);
                }
            } else {
                result[0] = self;
            }
            return result;
        } else {
            let mut result = vec![Self::ZERO; relative.y.abs() as usize + 1];
            let slope = relative.x as f64 / (relative.y as f64);
            if relative.y > 0 {
                for i in 0..=relative.y {
                    result[i as usize] =
                        Self::new(self.x + (i as f64 * slope).round() as i32, self.y + i);
                }
            } else if relative.y < 0 {
                for i in 0..=-relative.y {
                    result[i as usize] =
                        Self::new(self.x - (i as f64 * slope).round() as i32, self.y - i);
                }
            } else {
                result[0] = self;
            }
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_new() {
        let p = IntPoint::new(2, 3);
        assert_eq!(2, p.x);
        assert_eq!(3, p.y);
    }
    #[test]
    fn test_area() {
        assert_eq!(6, IntPoint::new(2, 3).area());
    }
    #[test]
    fn test_length_sq() {
        assert_eq!(5 * 5, IntPoint::new(3, 4).length_sq());
        assert_eq!(5 * 5, IntPoint::new(-3, -4).length_sq());
    }
    #[test]
    fn test_contains() {
        let p = IntPoint::new(3, 4);
        assert!(p.contains(IntPoint::new(1, 1)));
        assert!(p.contains(IntPoint::new(0, 0)));
        assert!(p.contains(IntPoint::new(2, 3)));
        assert!(p.contains(IntPoint::new(0, 3)));
        assert!(p.contains(IntPoint::new(2, 0)));
        assert!(!p.contains(IntPoint::new(-1, 1)));
        assert!(!p.contains(IntPoint::new(1, -1)));
        assert!(!p.contains(IntPoint::new(-2, -3)));
        assert!(!p.contains(IntPoint::new(1, 4)));
        assert!(!p.contains(IntPoint::new(3, 1)));
        assert!(!p.contains(IntPoint::new(1, 7)));
        assert!(!p.contains(IntPoint::new(5, 1)));
        assert!(!p.contains(IntPoint::new(8, 9)));
    }
    #[test]
    fn test_add() {
        assert_eq!(
            IntPoint::new(6, 8),
            IntPoint::new(2, 3) + IntPoint::new(4, 5)
        );
    }
    #[test]
    fn test_sub() {
        assert_eq!(
            IntPoint::new(2, 3),
            IntPoint::new(6, 8) - IntPoint::new(4, 5)
        );
    }
    #[test]
    fn test_neg() {
        assert_eq!(IntPoint::new(-2, -3), -IntPoint::new(2, 3));
    }
    #[test]
    fn test_to_double_point() {
        double_point::assert_eq(
            DoublePoint::new(2.0, 3.0),
            IntPoint::new(2, 3).to_double_point(),
            0.001,
        );
    }
    #[test]
    fn test_eq() {
        assert!(IntPoint::new(2, 3) == IntPoint::new(2, 3));
        assert!(IntPoint::new(2, 3) != IntPoint::new(0, 3));
        assert!(IntPoint::new(2, 3) != IntPoint::new(2, 0));
    }
    fn calculate_hash(p: IntPoint) -> u64 {
        let mut s = DefaultHasher::new();
        p.hash(&mut s);
        s.finish()
    }
    #[test]
    fn test_hash() {
        assert_eq!(
            calculate_hash(IntPoint::new(2, 3)),
            calculate_hash(IntPoint::new(2, 3))
        );
        assert_ne!(
            calculate_hash(IntPoint::new(2, 3)),
            calculate_hash(IntPoint::new(-2, 3))
        );
        assert_ne!(
            calculate_hash(IntPoint::new(2, 3)),
            calculate_hash(IntPoint::new(2, -3))
        );
    }
    #[test]
    fn test_edge_neighbors() {
        let mut s = HashSet::new();
        for n in IntPoint::EDGE_NEIGHBORS.iter() {
            s.insert(n);
            assert_eq!(1, n.length_sq());
        }
        assert_eq!(4, s.len());
    }
    #[test]
    fn test_corner_neighbors() {
        let mut s = HashSet::new();
        for n in IntPoint::CORNER_NEIGHBORS.iter() {
            s.insert(n);
            assert!(n.length_sq() == 1 || n.length_sq() == 2);
        }
        assert_eq!(8, s.len());
    }
    #[test]
    fn test_iter() {
        let mut l = Vec::new();
        for p in IntPoint::new(2, 3) {
            l.push(p);
        }
        assert_eq!(
            l,
            [
                IntPoint::new(0, 0),
                IntPoint::new(1, 0),
                IntPoint::new(0, 1),
                IntPoint::new(1, 1),
                IntPoint::new(0, 2),
                IntPoint::new(1, 2)
            ]
        );
        for _ in IntPoint::new(0, 3) {
            panic!();
        }
        for _ in IntPoint::new(3, 0) {
            panic!();
        }
        for _ in IntPoint::new(-1, 3) {
            panic!();
        }
        for _ in IntPoint::new(3, -1) {
            panic!();
        }
    }
    #[test]
    fn test_line_to() {
        check_line_to(2, 3, 2, 3, &[2, 3]);
        check_line_to(2, 3, 1, 4, &[2, 3, 1, 4]);
        check_line_to(2, 3, -1, 3, &[2, 3, 1, 3, 0, 3, -1, 3]);
        check_line_to(-1, 2, 0, -1, &[-1, 2, -1, 1, 0, 0, 0, -1]);
        check_line_to(1, 1, 3, 7, &[1, 1, 1, 2, 2, 3, 2, 4, 2, 5, 3, 6, 3, 7]);
        check_line_to(1, 3, 6, 1, &[1, 3, 2, 3, 3, 2, 4, 2, 5, 1, 6, 1]);
    }
    fn check_line_to(x1: i32, y1: i32, x2: i32, y2: i32, p: &[i32]) {
        let mut l = Vec::new();
        for i in 0..p.len() / 2 {
            l.push(IntPoint::new(p[2 * i], p[2 * i + 1]));
        }
        assert_eq!(l, IntPoint::new(x1, y1).line_to(IntPoint::new(x2, y2)));
    }
}
