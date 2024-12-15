use super::Parser;
use libadvent::{
    grid::{Direction, Grid, Point},
    AsInput,
};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    Box,
    Wall,
    Robot,
}

impl AsInput for Square {
    type Input = Self;

    fn from_str(s: &str) -> Self::Input {
        s.chars()
            .map(|c| match c {
                '.' => Square::Empty,
                '#' => Square::Wall,
                'O' => Square::Box,
                '@' => Square::Robot,
                _ => panic!("invalid square"),
            })
            .next()
            .unwrap()
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Square::Empty => '.',
            Square::Wall => '#',
            Square::Box => 'O',
            Square::Robot => '@',
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    squares: Grid<Square>,
    robot: Point,
    moves: Vec<Direction>, // NOTE TO SELF: BACKWARDS
}

impl Input {
    fn shift(&mut self, n: Point, to: Direction) -> bool {
        let next = n.offset(*to);

        if !self.squares.inbounds(next) {
            return false;
        }

        // cannot be moved
        if self.squares[next] == Square::Wall {
            return false;
        }

        // should short-circuit if not a box
        if self.squares[next] == Square::Box && !self.shift(next, to) {
            return false;
        }

        // will be empty if it was a box (see above)
        if self.squares[next] == Square::Empty {
            self.squares.swap(n, next);
        }

        true
    }

    fn step(&mut self) -> bool {
        let Some(next) = self.moves.pop() else {
            return false;
        };

        // if shift possible
        if self.shift(self.robot, next) {
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
            .filter(|(_, t)| *t == Square::Box)
            .map(|(p, _)| (p.y() * 100) + p.x())
            .sum::<isize>() as usize
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.squares.fmt(f)
    }
}

pub fn level1(input: <Parser as AsInput>::Input) -> usize {
    let (squares, robot, moves) = input;
    let mut input = Input {
        squares,
        robot,
        moves,
    };

    input.run();
    input.collect()
}
