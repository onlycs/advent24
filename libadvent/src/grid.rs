// everything is y,x here
use crate::{IsInput, Parser};
use itertools::Itertools;
use std::{
    cmp, fmt,
    iter::{Enumerate, Flatten, Map},
    ops::{Add, AddAssign, Deref, Index, IndexMut, Mul, Neg, Sub, SubAssign},
    ptr,
};

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

    pub fn as_usize(&self) -> Option<(usize, usize)> {
        if self.0 < 0 || self.1 < 0 {
            None
        } else {
            Some((self.0 as usize, self.1 as usize))
        }
    }

    pub fn as_usize_lim(&self, (liny, limx): (usize, usize)) -> Option<(usize, usize)> {
        let (y, x) = self.as_usize()?;

        if y >= liny || x >= limx {
            None
        } else {
            Some((y, x))
        }
    }

    pub fn from_1d(other: usize, width: usize) -> Self {
        let y = other / width;
        let x = other % width;

        Self(y as isize, x as isize)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Deref for Direction {
    type Target = Offset;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Up => &Offset(-1, 0),
            Self::Down => &Offset(1, 0),
            Self::Left => &Offset(0, -1),
            Self::Right => &Offset(0, 1),
        }
    }
}

impl IsInput for Direction {
    fn parse(s: &str) -> Self {
        match s {
            "^" => Self::Up,
            ">" => Self::Right,
            "v" => Self::Down,
            "<" => Self::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(init: T, height: usize, width: usize) -> Self
    where
        T: Clone,
    {
        let inner = vec![vec![init; width]; height];

        Self { inner }
    }

    pub fn size(&self) -> (usize, usize) {
        let y = self.inner.len();
        let x = self.inner[0].len();

        (y, x)
    }

    pub fn inbounds(&self, p: Point) -> bool {
        p.as_usize_lim(self.size()).is_some()
    }

    pub fn is(&self, p: Point, other: &T) -> bool
    where
        T: PartialEq,
    {
        self.inbounds(p) && &self[p] == other
    }

    pub fn is_point(&self, a: Point, b: Point) -> bool
    where
        T: PartialEq,
    {
        self.inbounds(b) && self.is(a, &self[b])
    }

    pub fn swap(&mut self, a: Point, b: Point) {
        if a == b || !self.inbounds(a) || !self.inbounds(b) {
            return;
        }

        unsafe {
            // SAFETY: a and b are in bounds and different
            ptr::swap(&mut self[a], &mut self[b]);
        }
    }

    pub fn find(&self, other: &T) -> Point
    where
        T: PartialEq,
    {
        for (y, row) in self.inner.iter().enumerate() {
            if let Some(x) = row.iter().position(|cell| cell == other) {
                return Point(y as isize, x as isize);
            }
        }

        panic!("Not found");
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.inner {
            for cell in row {
                write!(f, "{}", cell)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        let (y, x) = point
            .as_usize_lim(self.size())
            .expect("Point out of bounds");

        &self.inner[y][x]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let (y, x) = point
            .as_usize_lim(self.size())
            .expect("Point out of bounds");

        &mut self.inner[y][x]
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Point, T);
    type IntoIter = Map<
        Enumerate<Flatten<<Vec<Vec<T>> as IntoIterator>::IntoIter>>,
        impl Fn((usize, T)) -> (Point, T),
    >;

    fn into_iter(self) -> Self::IntoIter {
        let width = self.inner[0].len();
        let f = move |(i, t)| (Point::from_1d(i, width), t);

        self.inner.into_iter().flatten().enumerate().map(f)
    }
}

pub struct GridParser<T: Parser, F: FnMut(char, Point)> {
    inner: T,
    f: F,
}

impl<T: Parser> GridParser<T, fn(char, Point)> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            f: |_, _| (),
        }
    }
}

impl<T: Parser, F: FnMut(char, Point)> GridParser<T, F> {
    pub const fn with_f(inner: T, f: F) -> Self {
        Self { inner, f }
    }
}

impl<T: Parser, F: FnMut(char, Point)> Parser for GridParser<T, F> {
    type Input = Grid<T::Input>;

    fn parse(&mut self, s: &str) -> Self::Input {
        let inner = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        (self.f)(c, Point(i as isize, j as isize));
                        self.inner.parse(&c.to_string())
                    })
                    .collect_vec()
            })
            .collect_vec();

        Grid { inner }
    }
}

impl<T, I2: IntoIterator<Item = T>> FromIterator<I2> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = I2>>(iter: I) -> Self {
        let inner = iter
            .into_iter()
            .map(|row| row.into_iter().collect_vec())
            .collect_vec();

        let width = inner[0].len();

        assert!(
            inner.iter().all(|n| n.len() == width),
            "Subiterators do not have the same width"
        );

        Self { inner }
    }
}
