use itertools::Itertools;
use libadvent::{NewlineSeperated, Solution, WhiteSeperated};

pub struct _Solution;

fn abs_ok(abs: i32) -> bool {
    abs >= 1 && abs <= 3
}

fn ok(report: &[i32]) -> bool {
    let increasing = report[1] - report[0] > 0;

    report
        .array_windows::<2>()
        .into_iter()
        .all(|[a, b]| (b - a > 0) == increasing && abs_ok((a - b).abs()))
}

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

    return false;
}

impl Solution for _Solution {
    type Input = NewlineSeperated<WhiteSeperated<i32>>;
    type Output1 = usize;
    type Output2 = usize;

    fn day1(input: <Self::Input as libadvent::AsInput>::Input) -> Self::Output1 {
        input.into_iter().filter(|v| ok(v.as_slice())).count()
    }

    fn day2(input: <Self::Input as libadvent::AsInput>::Input) -> Self::Output2 {
        input.into_iter().filter(ok_skipping).count()
    }
}
