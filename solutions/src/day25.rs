use itertools::Itertools;
use libadvent::IsInput;

fn parse_bitmap(s: &[&str]) -> u32 {
    s.iter()
        .flat_map(|line| line.chars())
        .fold(0, |acc, ch| acc << 1 | (ch == '#') as u32)
}

pub struct Input {
    locks: Vec<u32>,
    keys: Vec<u32>,
}

impl IsInput for Input {
    fn parse(s: &str) -> Self {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for item in s.split("\n\n") {
            let lines = item.lines().collect_vec();
            let is_key = lines[0].starts_with("#");
            let end = lines.len() - 1;
            let b = parse_bitmap(&lines[1..end]);

            if is_key {
                keys.push(b);
            } else {
                locks.push(b);
            }
        }

        Self { locks, keys }
    }
}

problem_parser!(ty Input);

pub fn level1(combos: Input) -> usize {
    let mut fit = 0;

    for key in &combos.keys {
        for lock in &combos.locks {
            if key & lock == 0 {
                fit += 1;
            }
        }
    }

    fit
}

pub fn level2(_: Input) -> String {
    "Merry Christmas!".to_string()
}
