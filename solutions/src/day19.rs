use std::collections::HashMap;

use itertools::Itertools;
use libadvent::{IsInput, Parser, Seperated};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Input {
    parts: Vec<String>,
    towels: Vec<String>,
}

impl IsInput for Input {
    fn parse(s: &str) -> Self {
        let ins = s.split("\n\n").collect_vec();
        let parts = Seperated::new(", ", ty_parser!(String)).parse(ins[0]);
        let towels = Seperated::newline(ty_parser!(String)).parse(ins[1]);

        Self { parts, towels }
    }
}

impl Input {
    pub fn possible(&self, towel: &str) -> bool {
        if towel.is_empty() {
            return true;
        }

        for part in &self.parts {
            if towel.starts_with(part) && self.possible(&towel[part.len()..]) {
                return true;
            }
        }

        false
    }

    pub fn possible_ways(&self, towel: &str) -> usize {
        self.possible_ways_memoized(towel, &mut HashMap::new())
    }

    fn possible_ways_memoized(&self, towel: &str, memo: &mut HashMap<String, usize>) -> usize {
        if let Some(&count) = memo.get(towel) {
            return count;
        }

        if towel.is_empty() {
            return 1;
        }

        let mut ways = 0;
        for part in &self.parts {
            if towel.starts_with(part) {
                ways += self.possible_ways_memoized(&towel[part.len()..], memo);
            }
        }

        memo.insert(towel.to_string(), ways);
        ways
    }
}

problem_parser!(ty Input);

pub fn level1(input: Input) -> usize {
    input
        .towels
        .par_iter()
        .filter(|towel| input.possible(towel))
        .count()
}

pub fn level2(input: Input) -> usize {
    input
        .towels
        .par_iter()
        .map(|towel| input.possible_ways(towel))
        .sum::<usize>()
}
