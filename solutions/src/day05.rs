use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    collections::HashSet,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrderingRule {
    lhs: u32,
    rhs: u32,
}

impl FromStr for OrderingRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("|");
        let lhs = parts.next().unwrap().parse().unwrap();
        let rhs = parts.next().unwrap().parse().unwrap();

        Ok(Self { lhs, rhs })
    }
}

impl From<OrderingRule> for (u32, u32) {
    fn from(other: OrderingRule) -> (u32, u32) {
        (other.lhs, other.rhs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Update {
    nums: Vec<u32>,
}

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(",").map(|n| n.parse().unwrap()).collect();

        Ok(Self { nums })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    rules: HashSet<(u32, u32)>,
    updates: Vec<Update>,
}

impl Input {
    fn rules_containing(&self, n: u32) -> Vec<(u32, Ordering)> {
        self.rules
            .iter()
            .filter(|(a, b)| *a == n || *b == n)
            .map(|(a, b)| if *a == n { (*b, Greater) } else { (*a, Less) })
            .collect()
    }

    fn rules_both(&self, n: u32, m: u32) -> Option<Ordering> {
        self.rules
            .iter()
            .find(|ab| **ab == (n, m) || **ab == (m, n))
            .map(|(a, _)| if *a == n { Less } else { Greater })
    }

    fn follows(&self, i: usize) -> bool {
        let up = &self.updates[i];

        for j in 0..up.nums.len() {
            let r = self.rules_containing(up.nums[j]);

            for (other, side) in r {
                match side {
                    Less => {
                        if up.nums[j + 1..].contains(&other) {
                            return false;
                        }
                    }
                    Greater => {
                        if up.nums[..j].contains(&other) {
                            return false;
                        }
                    }
                    Equal => {
                        continue;
                    }
                }
            }
        }

        true
    }

    fn reorder(&mut self, i: usize) {
        let this = self.clone();

        self.updates[i].nums.sort_by(|a, b| {
            let ord = this.rules_both(*a, *b);

            match ord {
                Some(ord) => ord,
                None => a.cmp(b),
            }
        });
    }

    fn middle(&self, i: usize) -> u32 {
        self.updates[i].nums[self.updates[i].nums.len() / 2]
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");

        let rules = parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse::<OrderingRule>().unwrap().into())
            .collect();

        let updates = parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        Ok(Self { rules, updates })
    }
}

pub type Parser = Input;

pub fn level1(input: Input) -> u32 {
    let mut sum = 0;

    for i in 0..input.updates.len() {
        if input.follows(i) {
            sum += input.middle(i);
        }
    }

    sum
}

pub fn level2(mut input: Input) -> u32 {
    let mut sum = 0;

    for i in 0..input.updates.len() {
        if !input.follows(i) {
            input.reorder(i);
            sum += input.middle(i);
        }
    }

    sum
}
