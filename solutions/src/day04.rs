use itertools::Itertools;
use libadvent::{NewlineSeperated, Single};

pub type Input = NewlineSeperated<Single<char>>;

fn findxmas(
    input: &Vec<Vec<char>>,
    (mut i, mut j): (usize, usize),
    (offi, offj): (i32, i32),
) -> bool {
    let len = input.len();

    let ti = |i| {
        let newi = i as i32 + offi;
        if newi >= 0 && newi < len as i32 {
            Some(newi as usize)
        } else {
            None
        }
    };

    let tj = |j| {
        let newj = j as i32 + offj;
        if newj >= 0 && newj < input[0].len() as i32 {
            Some(newj as usize)
        } else {
            None
        }
    };

    let mut pattern = vec!['X', 'M', 'A', 'S'].into_iter().rev().collect_vec();

    while let Some(c) = pattern.pop() {
        if input[i][j] != c {
            return false;
        }

        let Some(newi) = ti(i) else {
            return pattern.len() == 0;
        };

        let Some(newj) = tj(j) else {
            return pattern.len() == 0;
        };

        i = newi;
        j = newj;
    }

    true
}

fn find_mas(input: &Vec<Vec<char>>, (i, j): (usize, usize)) -> bool {
    if i == 0 || j == 0 || i == input.len() - 1 || j == input[0].len() - 1 || input[i][j] != 'A' {
        return false;
    }

    let valids = vec!['M', 'S'];
    let diags = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)]
        .into_iter()
        .map(|(offi, offj)| (i as i32 + offi, j as i32 + offj))
        .map(|(i, j)| input[i as usize][j as usize])
        .collect_vec();

    if diags.iter().all(|c| valids.contains(c)) && diags[0] != diags[1] && diags[2] != diags[3] {
        return true;
    }

    false
}

pub fn level1(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            for offi in -1..=1 {
                for offj in -1..=1 {
                    if findxmas(&input, (i, j), (offi, offj)) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn level2(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            if find_mas(&input, (i, j)) {
                count += 1;
            }
        }
    }

    count
}
