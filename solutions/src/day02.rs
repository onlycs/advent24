use itertools::Itertools;
use libadvent::{NewlineSeperated, WhiteSeperated};

fn abs_ok(abs: i32) -> bool {
    (1..=3).contains(&abs)
}

fn ok(report: &[i32]) -> bool {
    let increasing = report[1] - report[0] > 0;

    report
        .array_windows::<2>()
        .into_iter()
        .all(|[a, b]| (b - a > 0) == increasing && abs_ok((a - b).abs()))
}

#[allow(clippy::ptr_arg)]
fn ok_skipping(report: &Vec<i32>) -> bool {
    // haaax
    for i in 0..report.len() {
        let report = report
            .iter()
            .enumerate()
            .filter_map(|(j, v)| if i == j { None } else { Some(*v) })
            .collect_vec();

        if ok(report.as_slice()) {
            return true;
        }
    }

    false
}

pub type Parser = NewlineSeperated<WhiteSeperated<i32>>;

pub fn level1(input: Vec<Vec<i32>>) -> usize {
    input.into_iter().filter(|v| ok(v.as_slice())).count()
}

pub fn level2(input: Vec<Vec<i32>>) -> usize {
    input.into_iter().filter(ok_skipping).count()
}
