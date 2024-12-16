use std::{collections::HashSet, sync::Arc};

use libadvent::{Seperated, Take};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

problem_parser!(Seperated::newline(Take::one(ty_parser!(u8))) => Vec<Vec<u8>>);

type ArcInput = Arc<[Arc<[u8]>]>;

fn arcify(input: Vec<Vec<u8>>) -> ArcInput {
    input.into_iter().map(|v| Arc::from(v.as_slice())).collect()
}

fn surrounding(i: usize, j: usize, imax: usize, jmax: usize) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();

    if i > 0 {
        vec.push((i - 1, j));
    }

    if j > 0 {
        vec.push((i, j - 1));
    }

    if i < imax {
        vec.push((i + 1, j));
    }

    if j < jmax {
        vec.push((i, j + 1));
    }

    vec
}

fn dfs(a: ArcInput, i: usize, j: usize, n: u8) -> HashSet<(usize, usize)> {
    if n == 10 {
        return HashSet::from_iter([(i, j)]);
    }

    surrounding(i, j, a.len() - 1, a[0].len() - 1)
        .into_par_iter()
        .filter(|(i, j)| a[*i][*j] == n)
        .map(|(i, j)| dfs(Arc::clone(&a), i, j, n + 1))
        .reduce(HashSet::new, |mut acc, set| {
            acc.extend(set);
            acc
        })
}

fn dfs2(a: ArcInput, i: usize, j: usize, n: u8) -> usize {
    if n == 10 {
        return 1;
    }

    surrounding(i, j, a.len() - 1, a[0].len() - 1)
        .into_par_iter()
        .filter(|(i, j)| a[*i][*j] == n)
        .map(|(i, j)| dfs2(Arc::clone(&a), i, j, n + 1))
        .sum::<usize>()
}

pub fn level1(input: Vec<Vec<u8>>) -> usize {
    let input = arcify(input);

    (0..input.len())
        .flat_map(|i| {
            (0..input[0].len())
                .map(move |j| (i, j))
                .filter(|(i, j)| input[*i][*j] == 0)
        })
        .par_bridge()
        .map(|(i, j)| dfs(Arc::clone(&input), i, j, 1).len())
        .sum::<usize>()
}

pub fn level2(input: Vec<Vec<u8>>) -> usize {
    let input = arcify(input);

    (0..input.len())
        .flat_map(|i| {
            (0..input[0].len())
                .map(move |j| (i, j))
                .filter(|(i, j)| input[*i][*j] == 0)
        })
        .par_bridge()
        .map(|(i, j)| dfs2(Arc::clone(&input), i, j, 1))
        .sum::<usize>()
}
