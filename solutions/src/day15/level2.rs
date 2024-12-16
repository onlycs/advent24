use std::fmt;

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
}

impl Square {
    fn other_direction(&self) -> Direction {
        match self {
            Square::BoxLeft => Direction::Right,
            Square::BoxRight => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Square::Empty => '.',
            Square::Wall => '#',
            Square::BoxLeft => '[',
            Square::BoxRight => ']',
            Square::Robot => '@',
        };

        write!(f, "{}", c)
    }
}

impl From<Square1> for [Square; 2] {
    fn from(s: Square1) -> [Square; 2] {
        match s {
            Square1::Empty => [Square::Empty, Square::Empty],
            Square1::Wall => [Square::Wall, Square::Wall],
            Square1::Box => [Square::BoxLeft, Square::BoxRight],
            Square1::Robot => [Square::Robot, Square::Empty],
        }
    }
}

fn fix_grid(v: Grid<Square1>) -> Grid<Square> {
    v.into_iter()
        .chunk_by(|(pt, _)| pt.y())
        .into_iter()
        .map(|(_, row)| row.into_iter().flat_map(|(_, s)| <[Square; 2]>::from(s)))
        .collect()
}

fn fix_robot(Point(y, x): Point) -> Point {
    Point(y, x * 2)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    squares: Grid<Square>,
    robot: Point,
    moves: Vec<Direction>, // NOTE TO SELF: BACKWARDS
}

impl Input {
    fn canshift(&mut self, n: Point, to: Direction) -> bool {
        let next = n.offset(*to);

        if !self.squares.inbounds(next) {
            return false;
        }

        match self.squares[next] {
            Square::Wall => false,
            Square::Empty => true,
            sq if matches!(to, Direction::Up | Direction::Down) => {
                self.canshift(next, to) && self.canshift(next.offset(*sq.other_direction()), to)
            }
            Square::BoxLeft | Square::BoxRight => self.canshift(next, to),
            _ => unreachable!(),
        }
    }

    fn shift(&mut self, n: Point, to: Direction) {
        let next = n.offset(*to);
        let sq = self.squares[next];

        if sq == Square::Empty {
            self.squares.swap(n, next);
        }

        if matches!(sq, Square::BoxLeft | Square::BoxRight)
            && matches!(to, Direction::Up | Direction::Down)
        {
            self.shift(next, to);
            self.shift(next.offset(*sq.other_direction()), to);

            // the lh/rhs of next is empty so only swap once
            self.squares.swap(n, next);
        } else if matches!(sq, Square::BoxLeft | Square::BoxRight) {
            self.shift(next, to);
            self.squares.swap(n, next);
        }
    }

    fn step(&mut self) -> bool {
        let Some(next) = self.moves.pop() else {
            return false;
        };

        if self.canshift(self.robot, next) {
            self.shift(self.robot, next);
            self.robot += *next;
        }

        true
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn collect(self) -> usize {
        self.squares
            .into_iter()
            .filter(|(_, t)| *t == Square::BoxLeft)
            .map(|(p, _)| (p.y() * 100) + p.x())
            .sum::<isize>() as usize
    }
}

pub fn level2(input: <InputParser as Parser>::Input) -> usize {
    let (squares, robot, moves) = input;

    let mut input = Input {
        squares: fix_grid(squares),
        robot: fix_robot(robot),
        moves,
    };

    input.run();
    input.collect()
}
