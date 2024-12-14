// everything is y,x here
use std::{
    cmp,
    ops::{Add, Neg, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub isize, pub isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Offset(pub isize, pub isize);

impl Point {
    pub const ORIGIN: Self = Self(0, 0);

    pub fn new(y: usize, x: usize) -> Self {
        Self(y as isize, x as isize)
    }

    pub fn offset(&self, offset: Offset) -> Self {
        let Offset(y, x) = offset;

        Self(self.0 + y, self.1 + x)
    }

    pub fn dist(&self, other: Point) -> f64 {
        let y = (self.0 - other.0).abs() as f64;
        let x = (self.1 - other.1).abs() as f64;

        (y * y + x * x).sqrt()
    }
}

impl Add<Offset> for Point {
    type Output = Self;

    fn add(self, offset: Offset) -> Self::Output {
        self.offset(offset)
    }
}

impl Sub<Offset> for Point {
    type Output = Self;

    fn sub(self, offset: Offset) -> Self::Output {
        self.offset(-offset)
    }
}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dist(Point::ORIGIN)
            .total_cmp(&other.dist(Point::ORIGIN))
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
