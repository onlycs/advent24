use std::collections::HashMap;

use itertools::Itertools;
use libadvent::{AsInput, WhiteSeperated};

pub type Parser = WhiteSeperated<u64>;
type Input = <Parser as AsInput>::Input;

fn solve(data: Input, steps: usize) -> usize {
    // store each number and the number of times it appears
    // im thinking with 237 trillion+ numbers, that's like
    // 8 bytes per u64 * 237 trillion u64s = 1.9 petabytes of memory
    // and with that many numbers, there's probably some repetition
    let mut counts = data.into_iter().fold(HashMap::new(), |mut acc, f| {
        acc.entry(f).and_modify(|n| *n += 1).or_insert(1usize);
        acc
    });

    for i in 0..steps {
        println!("step {i}");

        for (num, occurrences) in counts.drain().collect_vec() {
            let mut add = |num: u64| {
                counts
                    .entry(num)
                    .and_modify(|n| *n += occurrences)
                    .or_insert(occurrences);
            };

            if num == 0 {
                add(num + 1);
            } else {
                let dplaces = ((num as f64).log10() as u32) + 1;
                if dplaces % 2 == 0 {
                    let lhs = num / 10u64.pow(dplaces / 2);
                    let rhs = num % 10u64.pow(dplaces / 2);
                    add(lhs);
                    add(rhs);
                } else {
                    add(num * 2024);
                }
            }
        }
    }

    counts.values().sum()
}

pub fn level1(data: Input) -> usize {
    solve(data, 25)
}

pub fn level2(data: Input) -> usize {
    solve(data, 75)
}
