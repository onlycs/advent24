#![feature(impl_trait_in_assoc_type, pattern)]

use std::{
    fmt, marker,
    str::{pattern::Pattern, FromStr},
};

use itertools::Itertools;

pub mod grid;

#[macro_export]
macro_rules! problem_parser {
    ($p:expr => $ty:ty) => {
        pub fn parser() -> impl ::libadvent::Parser<Output = $ty> {
            $p
        }
    };

    (parser $p:tt) => {
        pub fn parser() -> $p {
            $p
        }
    };

    (ty $p:tt) => {
        pub fn parser() -> impl ::libadvent::Parser<Output = $p> {
            ty_parser!($p)
        }
    };

    ($p:expr) => {
        mod __parser {
            use super::*;
            problem_parser!($p => super::Input);
        }

        pub use __parser::parser;
    };
}

#[macro_export]
macro_rules! ty_parser {
    ($t:ty) => {
        ::libadvent::TyParser::<$t>::default()
    };
}

pub trait Parser {
    type Output;
    fn parse(&mut self, s: &str) -> Self::Output;
}

pub trait IsInput {
    fn parse(s: &str) -> Self;
}

pub struct TyParser<T: IsInput>(marker::PhantomData<T>);

impl<T: IsInput> Parser for TyParser<T> {
    type Output = T;

    fn parse(&mut self, s: &str) -> Self::Output {
        T::parse(s.trim())
    }
}

impl<T: IsInput> Default for TyParser<T> {
    fn default() -> Self {
        Self(marker::PhantomData)
    }
}

impl<T: FromStr<Err: fmt::Debug>> IsInput for T {
    fn parse(s: &str) -> Self {
        s.parse().expect("Failed to parse")
    }
}

pub struct Seperated<T: Parser, P: Pattern> {
    seperator: P,
    inner: T,
}

impl<T: Parser> Seperated<T, &'static str> {
    pub const fn new(seperator: &'static str, inner: T) -> Self {
        Self { seperator, inner }
    }

    pub const fn comma(inner: T) -> Self {
        Self::new(",", inner)
    }

    pub const fn newline(inner: T) -> Self {
        Self::new("\n", inner)
    }
}

impl<T: Parser> Seperated<T, fn(char) -> bool> {
    pub const fn whitespace(inner: T) -> Self {
        Self {
            seperator: char::is_whitespace,
            inner,
        }
    }
}

impl<T: Parser, P: Pattern + Clone> Parser for Seperated<T, P> {
    type Output = Vec<T::Output>;

    fn parse(&mut self, s: &str) -> Self::Output {
        s.split(self.seperator.clone())
            .filter(|s| !s.is_empty())
            .map(|s| self.inner.parse(s))
            .collect()
    }
}

pub struct Take<T: Parser> {
    n: usize,
    inner: T,
}

impl<T: Parser> Take<T> {
    pub const fn new(n: usize, inner: T) -> Self {
        Self { n, inner }
    }

    pub const fn one(inner: T) -> Self {
        Self::new(1, inner)
    }
}

impl<T: Parser> Parser for Take<T> {
    type Output = Vec<T::Output>;

    fn parse(&mut self, s: &str) -> Self::Output {
        s.chars()
            .chunks(self.n)
            .into_iter()
            .map(|c| self.inner.parse(&c.collect::<String>()))
            .collect()
    }
}

pub struct Reverse<P: Parser<Output = Vec<T>>, T> {
    inner: P,
}

impl<P: Parser<Output = Vec<T>>, T> Reverse<P, T> {
    pub const fn new(inner: P) -> Self {
        Self { inner }
    }
}

impl<P: Parser<Output = Vec<T>>, T> Parser for Reverse<P, T> {
    type Output = Vec<T>;

    fn parse(&mut self, s: &str) -> Self::Output {
        let mut inner = self.inner.parse(s);
        inner.reverse();

        inner
    }
}
