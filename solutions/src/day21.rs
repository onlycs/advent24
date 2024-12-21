use std::{cmp, collections::HashMap, hash::Hash, iter};

use itertools::Itertools;
use lazy_static::lazy_static;
use libadvent::{
    grid::{Direction, Point},
    IsInput, Seperated, Take,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumBtn {
    Num(usize),
    Enter,
    Wall,
}

impl IsInput for NumBtn {
    fn parse(s: &str) -> Self {
        let ch = s.chars().collect_vec()[0];

        match ch {
            ch if ch.is_numeric() => Self::Num(ch as usize - ('0' as usize)),
            'A' => NumBtn::Enter,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArrowBtn {
    Dir(Direction),
    Enter,
    Wall,
}

lazy_static! {
    static ref NUMPOINTS: HashMap<NumBtn, Point> = HashMap::from_iter([
        (NumBtn::Num(0), Point::new(3, 1)),
        (NumBtn::Num(1), Point::new(2, 0)),
        (NumBtn::Num(2), Point::new(2, 1)),
        (NumBtn::Num(3), Point::new(2, 2)),
        (NumBtn::Num(4), Point::new(1, 0)),
        (NumBtn::Num(5), Point::new(1, 1)),
        (NumBtn::Num(6), Point::new(1, 2)),
        (NumBtn::Num(7), Point::new(0, 0)),
        (NumBtn::Num(8), Point::new(0, 1)),
        (NumBtn::Num(9), Point::new(0, 2)),
        (NumBtn::Enter, Point::new(3, 2)),
    ]);
    static ref ARROWPOINTS: HashMap<ArrowBtn, Point> = HashMap::from_iter([
        (ArrowBtn::Dir(Direction::Up), Point::new(0, 1)),
        (ArrowBtn::Dir(Direction::Left), Point::new(1, 0)),
        (ArrowBtn::Dir(Direction::Down), Point::new(1, 1)),
        (ArrowBtn::Dir(Direction::Right), Point::new(1, 2)),
        (ArrowBtn::Enter, Point::new(0, 2)),
    ]);
}

#[memoize::memoize] // if i'd known omg
pub fn calculate_arrow(src: Point, dest: Point, yfirst: bool, recursions: usize) -> usize {
    let dx = dest.x() - src.x();
    let dy = dest.y() - src.y();

    let mut path = Vec::new();

    if yfirst {
        path.extend(vec![
            ArrowBtn::Dir(if dy < 0 {
                Direction::Up
            } else {
                Direction::Down
            });
            dy.unsigned_abs()
        ]);

        path.extend(vec![
            ArrowBtn::Dir(if dx < 0 {
                Direction::Left
            } else {
                Direction::Right
            });
            dx.unsigned_abs()
        ]);
    } else {
        path.extend(vec![
            ArrowBtn::Dir(if dx < 0 {
                Direction::Left
            } else {
                Direction::Right
            });
            dx.unsigned_abs()
        ]);

        path.extend(vec![
            ArrowBtn::Dir(if dy < 0 {
                Direction::Up
            } else {
                Direction::Down
            });
            dy.unsigned_abs()
        ]);
    }

    path.push(ArrowBtn::Enter);

    if recursions == 0 {
        path.len()
    } else {
        iter::once(&ArrowBtn::Enter)
            .chain(path.iter())
            .tuple_windows()
            .map(|(src, dest)| {
                let src = ARROWPOINTS[src];
                let dest = ARROWPOINTS[dest];

                if src == Point::new(1, 0) {
                    // force horizontal first
                    calculate_arrow(src, dest, false, recursions - 1)
                } else if dest == Point::new(1, 0) {
                    // force vertical first
                    calculate_arrow(src, dest, true, recursions - 1)
                } else {
                    // check via min
                    cmp::min(
                        calculate_arrow(src, dest, true, recursions - 1),
                        calculate_arrow(src, dest, false, recursions - 1),
                    )
                }
            })
            .sum()
    }
}

pub fn calculate(nums: &[NumBtn], recs: usize) -> usize {
    iter::once(&NumBtn::Enter)
        .chain(nums.iter())
        .tuple_windows()
        .map(|(src, dest)| {
            let src = NUMPOINTS[src];
            let dest = NUMPOINTS[dest];

            if src.y() == 3 && dest.x() == 0 {
                calculate_arrow(src, dest, true, recs)
            } else if src.x() == 0 && dest.y() == 3 {
                calculate_arrow(src, dest, false, recs)
            } else {
                cmp::min(
                    calculate_arrow(src, dest, true, recs),
                    calculate_arrow(src, dest, false, recs),
                )
            }
        })
        .sum()
}

fn get_num(list: &[NumBtn]) -> usize {
    let numplaces = list[..list.len() - 1]
        .iter()
        .filter_map(|n| {
            let NumBtn::Num(n) = n else { return None };
            Some(*n)
        })
        .collect_vec();

    numplaces[2] + (numplaces[1] * 10) + (numplaces[0] * 100)
}

problem_parser!(Seperated::newline(Take::one(ty_parser!(NumBtn))));
type Input = Vec<Vec<NumBtn>>;

pub fn level1(input: Input) -> usize {
    input
        .into_iter()
        .map(|num| calculate(&num, 2) * get_num(&num))
        .sum()
}

pub fn level2(input: Input) -> usize {
    input
        .into_iter()
        .map(|num| calculate(&num, 25) * get_num(&num))
        .sum()
}
