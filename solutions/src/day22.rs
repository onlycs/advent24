use std::collections::{HashMap, HashSet};

use libadvent::Seperated;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

problem_parser!(Seperated::newline(ty_parser!(isize)));
type Input = Vec<isize>;

pub fn next_secret(mut n: isize) -> isize {
    // mix n by n*64
    n ^= n * 64;

    // prune
    n %= 16777216;

    // mix n by n/32
    n ^= n / 32;

    // prune
    n %= 16777216;

    // mix n by n*2048
    n ^= n * 2048;

    // prune
    n %= 16777216;

    n
}

pub fn next_2000(n: isize) -> isize {
    let mut n = n;
    for _ in 0..2000 {
        n = next_secret(n);
    }
    n
}

pub fn find_seqs(n: isize) -> HashMap<[isize; 4], isize> {
    let mut last = n;
    let mut costs = vec![n % 10];
    let mut delta = vec![];

    for _ in 0..2000 {
        last = next_secret(last);
        costs.push(last % 10);
        delta.push(*costs.last().unwrap() - costs[costs.len() - 2]);
    }

    let mut seqs = HashMap::new();

    for i in 0..delta.len() - 4 {
        let seq = <[isize; 4]>::try_from(&delta[i..i + 4]).unwrap();
        let bananas = costs[i + 4];

        seqs.entry(seq).or_insert(bananas);
    }

    seqs
}

pub fn level1(inputs: Input) -> isize {
    inputs.into_par_iter().map(next_2000).sum::<isize>()
}

pub fn level2(inputs: Input) -> isize {
    let seq_maps = inputs.into_par_iter().map(find_seqs).collect::<Vec<_>>();

    let all_seqs = seq_maps
        .par_iter()
        .fold(HashSet::<[isize; 4]>::new, |mut acc, next| {
            acc.extend(next.keys());
            acc
        })
        .reduce(HashSet::<[isize; 4]>::new, |acc, next| {
            acc.union(&next).copied().collect()
        });

    all_seqs
        .into_par_iter()
        .map(|seq| {
            seq_maps
                .par_iter()
                .map(|map| map.get(&seq).copied().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap()
}
