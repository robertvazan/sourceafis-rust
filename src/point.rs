// Part of SourceAFIS for Rust: https://sourceafis.machinezoo.com/rust

use super::*;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T> {
    x: T,
    y: T,
}

pub type IntPoint = Point<i32>;
pub type DoublePoint = Point<f64>;

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default + Copy + PartialOrd + Add<Output = T> + Mul<Output = T>> Point<T> {
    pub fn area(self) -> T {
        self.x * self.y
    }
    pub fn length_sq(self) -> T {
        number::sq(self.x) + number::sq(self.y)
    }
    pub fn contains(self, other: Self) -> bool {
        other.x >= T::default() && other.y >= T::default() && other.x < self.x && other.y < self.y
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl IntPoint {
    pub const ZERO: Self = Self::new(0, 0);
    pub const EDGE_NEIGHBORS: [Self; 4] = [
        Self::new(0, -1),
        Self::new(-1, 0),
        Self::new(1, 0),
        Self::new(0, 1),
    ];
    pub const CORNER_NEIGHBORS: [Self; 8] = [
        Self::new(-1, -1),
        Self::new(0, -1),
        Self::new(1, -1),
        Self::new(-1, 0),
        Self::new(1, 0),
        Self::new(-1, 1),
        Self::new(0, 1),
        Self::new(1, 1),
    ];

    pub fn to_double(self) -> Point<f64> {
        Point::new(self.x as f64, self.y as f64)
    }
    pub fn line_to(self, to: Self) -> Vec<Self> {
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

pub struct IntPointIterator {
    range: IntPoint,
    at: IntPoint,
}

impl Iterator for IntPointIterator {
    type Item = IntPoint;

    fn next(&mut self) -> Option<Self::Item> {
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
            at: Self::ZERO,
        }
    }
}

impl DoublePoint {
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_new() {
        let p = Point::new(2, 3);
        assert_eq!(2, p.x);
        assert_eq!(3, p.y);
    }
    #[test]
    fn test_area() {
        assert_eq!(6, Point::new(2, 3).area());
    }
    #[test]
    fn test_length_sq() {
        assert_eq!(5 * 5, Point::new(3, 4).length_sq());
        assert_eq!(5 * 5, Point::new(-3, -4).length_sq());
    }
    #[test]
    fn test_contains() {
        let p = Point::new(3, 4);
        assert!(p.contains(Point::new(1, 1)));
        assert!(p.contains(Point::new(0, 0)));
        assert!(p.contains(Point::new(2, 3)));
        assert!(p.contains(Point::new(0, 3)));
        assert!(p.contains(Point::new(2, 0)));
        assert!(!p.contains(Point::new(-1, 1)));
        assert!(!p.contains(Point::new(1, -1)));
        assert!(!p.contains(Point::new(-2, -3)));
        assert!(!p.contains(Point::new(1, 4)));
        assert!(!p.contains(Point::new(3, 1)));
        assert!(!p.contains(Point::new(1, 7)));
        assert!(!p.contains(Point::new(5, 1)));
        assert!(!p.contains(Point::new(8, 9)));
    }
    #[test]
    fn test_add() {
        assert_eq!(Point::new(6, 8), Point::new(2, 3) + Point::new(4, 5));
    }
    #[test]
    fn test_sub() {
        assert_eq!(Point::new(2, 3), Point::new(6, 8) - Point::new(4, 5));
    }
    #[test]
    fn test_neg() {
        assert_eq!(Point::new(-2, -3), -Point::new(2, 3));
    }
    fn assert_approx_point_eq(expected: DoublePoint, actual: DoublePoint, tolerance: f64) {
        assert_approx_eq(expected.x, actual.x, tolerance);
        assert_approx_eq(expected.y, actual.y, tolerance);
    }
    #[test]
    fn test_to_double_point() {
        assert_approx_point_eq(Point::new(2.0, 3.0), Point::new(2, 3).to_double(), 0.001);
    }
    #[test]
    fn test_eq() {
        assert!(Point::new(2, 3) == Point::new(2, 3));
        assert!(Point::new(2, 3) != Point::new(0, 3));
        assert!(Point::new(2, 3) != Point::new(2, 0));
    }
    fn calculate_hash<T: Hash>(p: Point<T>) -> u64 {
        let mut s = DefaultHasher::new();
        p.hash(&mut s);
        s.finish()
    }
    #[test]
    fn test_hash() {
        assert_eq!(
            calculate_hash(Point::new(2, 3)),
            calculate_hash(Point::new(2, 3))
        );
        assert_ne!(
            calculate_hash(Point::new(2, 3)),
            calculate_hash(Point::new(-2, 3))
        );
        assert_ne!(
            calculate_hash(Point::new(2, 3)),
            calculate_hash(Point::new(2, -3))
        );
    }
    #[test]
    fn test_edge_neighbors() {
        let mut s = HashSet::new();
        for n in Point::EDGE_NEIGHBORS.iter() {
            s.insert(n);
            assert_eq!(1, n.length_sq());
        }
        assert_eq!(4, s.len());
    }
    #[test]
    fn test_corner_neighbors() {
        let mut s = HashSet::new();
        for n in Point::CORNER_NEIGHBORS.iter() {
            s.insert(n);
            assert!(n.length_sq() == 1 || n.length_sq() == 2);
        }
        assert_eq!(8, s.len());
    }
    #[test]
    fn test_iter() {
        let mut l = Vec::new();
        for p in Point::new(2, 3) {
            l.push(p);
        }
        assert_eq!(
            l,
            [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(0, 2),
                Point::new(1, 2)
            ]
        );
        for _ in Point::new(0, 3) {
            panic!();
        }
        for _ in Point::new(3, 0) {
            panic!();
        }
        for _ in Point::new(-1, 3) {
            panic!();
        }
        for _ in Point::new(3, -1) {
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
            l.push(Point::new(p[2 * i], p[2 * i + 1]));
        }
        assert_eq!(l, Point::new(x1, y1).line_to(Point::new(x2, y2)));
    }
}
