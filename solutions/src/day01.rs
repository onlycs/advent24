use std::collections::HashMap;

use itertools::Itertools;
use libadvent::{Seperated, TyParser};

problem_parser!(Seperated::newline(Seperated::whitespace(
    TyParser::<i32>::default()
)) => Vec<Vec<i32>>);

pub fn level1(input: Vec<Vec<i32>>) -> i32 {
    let mut side1 = input.iter().map(|v| v[0]).collect_vec();
    let mut side2 = input.iter().map(|v| v[1]).collect_vec();

    side1.sort();
    side2.sort();

    let mut sum = 0;
    for (a, b) in side1.iter().zip(side2.iter()) {
        let diff = *a - *b;
        sum += diff.abs();
    }

    sum
}

pub fn level2(input: Vec<Vec<i32>>) -> i32 {
    let side1 = input.iter().map(|v| v[0]).collect_vec();
    let side2 = input
        .iter()
        .map(|v| v[1])
        .fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).and_modify(|e| *e += 1).or_insert(1);
            acc
        });

    let mut sum = 0;
    for a in side1.iter() {
        let appearances = side2.get(a).copied().unwrap_or(0);
        let similarity = a * appearances;
        sum += similarity;
    }

    sum
}
