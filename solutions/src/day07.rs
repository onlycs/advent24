use rayon::iter::{ParallelBridge, ParallelIterator};

use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub struct NnaryCounter<const N: u8> {
    digits: Vec<u8>,
    as_num: u64,
}

impl<const N: u8> NnaryCounter<N> {
    pub fn new(len: usize) -> Self {
        Self {
            digits: vec![0; len],
            as_num: 0,
        }
    }

    pub fn to_number(&self) -> u64 {
        self.digits
            .iter()
            .fold(0, |acc, &d| acc * N as u64 + d as u64)
    }

    pub fn from_number(num: u64, len: usize) -> Self {
        let mut digits = Vec::with_capacity(len);
        let mut num = num;

        for _ in 0..len {
            digits.push((num % N as u64) as u8);
            num /= N as u64;
        }

        Self {
            digits,
            as_num: num,
        }
    }

    pub fn increment(&mut self) {
        let carry = 1;

        for d in self.digits.iter_mut() {
            *d += carry;

            if *d == N {
                *d = 0;
            } else {
                break;
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.digits.iter().all(|&d| d == N - 1)
    }

    pub fn digit(&self, i: usize) -> u8 {
        self.digits[i]
    }
}

impl<const N: u8> Iterator for NnaryCounter<N> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_full() {
            return None;
        }

        let num = self.to_number();
        self.increment();
        Some(num)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn from_bit(b: u8) -> Self {
        match b {
            0 => Self::Add,
            1 => Self::Mul,
            2 => Self::Concat,
            _ => unreachable!(),
        }
    }

    fn from_mask<const N: u8>(mask: &NnaryCounter<N>, len: usize) -> Vec<Self> {
        let mut operations = Vec::with_capacity(len);
        for i in 0..len {
            operations.push(Self::from_bit(mask.digit(i)));
        }
        operations
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Problem<const N: u8> {
    numbers: Vec<u64>,
    operations: NnaryCounter<N>,
    total: u64,
}

impl<const N: u8> Problem<N> {
    pub fn new(numbers: Vec<u64>, total: u64) -> Self {
        Self {
            operations: NnaryCounter::new(numbers.len()),
            numbers,
            total,
        }
    }

    pub fn check(&self) -> bool {
        let ops = Operation::from_mask(&self.operations, self.numbers.len() - 1);
        let mut iter = self.numbers.iter();
        let mut total = *iter.next().unwrap();

        for (op, num) in ops.into_iter().zip(iter) {
            match op {
                Operation::Add => total += num,
                Operation::Mul => total *= num,
                Operation::Concat => {
                    let dplaces = (*num as f64).log10() as u32 + 1;
                    total = total * 10u64.pow(dplaces) + num;
                }
            }
        }

        total == self.total
    }

    pub fn solve(&mut self) -> u64 {
        self.operations
            .clone()
            .par_bridge()
            .find_map_any(|op| {
                let mut this = self.clone();
                this.operations = NnaryCounter::from_number(op, self.numbers.len());

                if this.check() {
                    Some(this.total)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

impl<const N: u8> FromStr for Problem<N> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let total = parts.next().unwrap().parse()?;
        let numbers = parts
            .next()
            .unwrap()
            .split(' ')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Ok(Self::new(numbers, total))
    }
}

pub struct Input<const N: u8> {
    problems: Vec<Problem<N>>,
}

impl<const N: u8> FromStr for Input<N> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let problems = s.lines().map(str::parse).map(Result::unwrap).collect();

        Ok(Self { problems })
    }
}

pub type Parser<const N: usize> = Input<N>;

pub fn level1(mut data: Input<2>) -> usize {
    data.problems
        .iter_mut()
        .par_bridge()
        .map(Problem::solve)
        .sum::<u64>() as usize
}

pub fn level2(mut data: Input<3>) -> usize {
    data.problems
        .iter_mut()
        .par_bridge()
        .map(Problem::solve)
        .sum::<u64>() as usize
}
