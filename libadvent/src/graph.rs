// everything is y,x here
use std::{
    cmp,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub isize, pub isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Offset(pub isize, pub isize);

impl Offset {
    pub fn parse_xy(s: &str, sep: &str) -> Self {
        let s = s.split(sep).collect_vec();
        let x = s[0].parse().unwrap();
        let y = s[1].parse().unwrap();

        Self(y, x)
    }

    pub fn parse_yx(s: &str, sep: &str) -> Self {
        let s = s.split(sep).collect_vec();
        let y = s[0].parse().unwrap();
        let x = s[1].parse().unwrap();

        Self(y, x)
    }

    pub fn y(&self) -> isize {
        self.0
    }

    pub fn x(&self) -> isize {
        self.1
    }
}

impl Point {
    pub const ORIGIN: Self = Self(0, 0);

    pub fn new(y: usize, x: usize) -> Self {
        Self(y as isize, x as isize)
    }

    pub fn parse_xy(s: &str, sep: &str) -> Self {
        let s = s.split(sep).collect_vec();
        let x = s[0].parse().unwrap();
        let y = s[1].parse().unwrap();

        Self(y, x)
    }

    pub fn parse_yx(s: &str, sep: &str) -> Self {
        let s = s.split(sep).collect_vec();
        let y = s[0].parse().unwrap();
        let x = s[1].parse().unwrap();

        Self(y, x)
    }

    pub fn offset(&self, offset: Offset) -> Self {
        let Self(y, x) = self;
        let Offset(oy, ox) = offset;

        Self(y + oy, x + ox)
    }

    pub fn dist(&self, other: Point) -> f64 {
        let y = (self.0 - other.0).abs() as f64;
        let x = (self.1 - other.1).abs() as f64;

        (y * y + x * x).sqrt()
    }

    pub fn x(&self) -> isize {
        self.1
    }

    pub fn y(&self) -> isize {
        self.0
    }

    pub fn rem(&mut self, (y, x): (isize, isize)) {
        self.0 = self.0.rem_euclid(y);
        self.1 = self.1.rem_euclid(x);
    }
}

impl Add<Offset> for Point {
    type Output = Self;

    fn add(self, offset: Offset) -> Self::Output {
        self.offset(offset)
    }
}

impl AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        let sum = *self + rhs;
        *self = sum;
    }
}

impl Sub<Offset> for Point {
    type Output = Self;

    fn sub(self, offset: Offset) -> Self::Output {
        self.offset(-offset)
    }
}

impl SubAssign<Offset> for Point {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
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

impl Mul<isize> for Offset {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<usize> for Offset {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as isize, self.1 * rhs as isize)
    }
}
