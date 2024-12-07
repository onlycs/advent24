use std::{
    collections::HashSet,
    fmt,
    ops::{Index, IndexMut},
    str::FromStr,
};

use ethnum::U256 as u256;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn modify(&self, n: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (n.0 - 1, n.1),
            Self::Down => (n.0 + 1, n.1),
            Self::Left => (n.0, n.1 - 1),
            Self::Right => (n.0, n.1 + 1),
        }
    }

    fn r90(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn r90_mut(&mut self) {
        *self = self.r90();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Obstacle(bool),
    Guard(Direction),
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Obstacle(false)),
            "^" => Ok(Self::Guard(Direction::Up)),
            "v" => Ok(Self::Guard(Direction::Down)),
            "<" => Ok(Self::Guard(Direction::Left)),
            ">" => Ok(Self::Guard(Direction::Right)),
            _ => Err(format!("Invalid square: {}", s)),
        }
    }
}

impl Square {
    #[track_caller]
    fn as_guard(&self) -> Direction {
        match self {
            Self::Guard(dir) => *dir,
            _ => panic!(),
        }
    }

    fn as_guard_mut(&mut self) -> &mut Direction {
        match self {
            Self::Guard(dir) => dir,
            _ => panic!(),
        }
    }

    fn lookahead(&mut self) -> bool {
        match self {
            Self::Obstacle(b) => {
                *b = true;
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    squares: Vec<Vec<Square>>,
    guard: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard = (0, 0);
        let mut squares: Vec<Vec<Square>> = vec![];

        for line in s.split("\n") {
            let mut row = vec![];

            for ch in line.chars() {
                let sq = Square::from_str(ch.to_string().as_str())?;

                if let Square::Guard(_) = sq {
                    guard = (squares.len(), row.len());
                }

                row.push(sq);
            }

            squares.push(row);
        }

        Ok(Self {
            squares,
            guard,
            visited: HashSet::from_iter([guard]),
        })
    }
}

impl Index<(usize, usize)> for Input {
    type Output = Square;

    fn index(&self, (a, b): (usize, usize)) -> &Self::Output {
        &self.squares[a][b]
    }
}

impl IndexMut<(usize, usize)> for Input {
    fn index_mut(&mut self, (a, b): (usize, usize)) -> &mut Self::Output {
        &mut self.squares[a][b]
    }
}

impl Input {
    fn step(&mut self) -> bool {
        let guard = self.guard;
        let dir = self[guard].as_guard();
        let lookahead = self[dir.modify(guard)].lookahead();

        self.visited.insert(guard);

        if lookahead {
            self[guard].as_guard_mut().r90_mut();
        } else {
            self[guard] = Square::Empty;
            self[dir.modify(guard)] = Square::Guard(dir);
            self.guard = dir.modify(guard);
        }

        !(!(1..self.squares.len() - 1).contains(&self.guard.0)
            || !(1..self.squares[0].len() - 1).contains(&self.guard.1))
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn visited(&self) -> usize {
        self.visited.len() + 1
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.squares {
            writeln!(f)?;

            for sq in row {
                match sq {
                    Square::Empty => write!(f, "."),
                    Square::Obstacle(_) => write!(f, "#"),
                    Square::Guard(Direction::Up) => write!(f, "^"),
                    Square::Guard(Direction::Down) => write!(f, "v"),
                    Square::Guard(Direction::Left) => write!(f, "<"),
                    Square::Guard(Direction::Right) => write!(f, ">"),
                }?;
            }
        }

        Ok(())
    }
}

// instead of storing two bytes per square, we can store two bits per square
#[derive(Clone, Debug, PartialEq)]
pub struct Bitgame {
    rows: Vec<u256>,
    cols: Vec<u256>,
    guard: (usize, usize),
    direction: Direction,
}

impl From<Input> for Bitgame {
    fn from(value: Input) -> Self {
        let direction = value[value.guard].as_guard();
        let guard = value.guard;
        let width = value.squares[0].len();
        let mut rows = vec![];

        for row in value.squares {
            let mut bm = u256::ZERO;

            for (i, sq) in row.into_iter().enumerate() {
                let value = match sq {
                    Square::Obstacle(_) => u256::from(1u128),
                    _ => u256::ZERO,
                };

                bm |= value << i;
            }

            rows.push(bm)
        }

        let mut cols = vec![u256::ZERO; width];

        for (row, rowmask) in rows.iter().enumerate() {
            for (col, colmask) in cols.iter_mut().enumerate() {
                let value = match rowmask & (u256::from(1u128) << col) {
                    u256::ZERO => u256::ZERO,
                    _ => u256::from(1u128),
                };

                *colmask |= value << row;
            }
        }

        Self {
            rows,
            cols,
            guard,
            direction,
        }
    }
}

impl From<Bitgame> for Input {
    fn from(value: Bitgame) -> Self {
        let mut squares = vec![];

        for row in value.rows {
            let mut r = vec![];

            for i in 0..value.cols.len() {
                let sq = match row & (u256::from(1u128) << i) {
                    u256::ZERO => Square::Empty,
                    _ => Square::Obstacle(false),
                };

                r.push(sq);
            }

            squares.push(r);
        }

        let guard = value.guard;
        squares[guard.0][guard.1] = Square::Guard(value.direction);

        Self {
            squares,
            guard,
            visited: HashSet::from_iter([guard]),
        }
    }
}

impl Bitgame {
    fn step(&mut self) -> bool {
        let (guardy, guardx) = self.guard;

        self.guard = match self.direction {
            Direction::Up => {
                let col = self.cols[guardx];

                let relevant = col & (u256::from(1u128) << guardy) - 1;
                let travel = (relevant << (256 - guardy)).leading_zeros();

                if guardy < travel as usize {
                    return false;
                }

                (guardy - travel as usize, guardx)
            }
            Direction::Down => {
                let col = self.cols[guardx];
                let relevant = col & !((u256::from(1u128) << guardy + 1) - 1);
                let travel = (relevant >> (guardy + 1)).trailing_zeros();
                (guardy + travel as usize, guardx)
            }
            Direction::Left => {
                let row = self.rows[guardy];
                let relevant = row & (u256::from(1u128) << guardx) - 1;
                let travel = (relevant << (256 - guardx)).leading_zeros();

                if guardx < travel as usize {
                    return false;
                }

                (guardy, guardx - travel as usize)
            }
            Direction::Right => {
                let row = self.rows[guardy];
                let relevant = row & !((u256::from(1u128) << guardx + 1) - 1);
                let travel = (relevant >> (guardx + 1)).trailing_zeros();
                (guardy, guardx + travel as usize)
            }
        };

        self.direction.r90_mut();

        !(self.guard.0 >= self.rows.len() || self.guard.1 >= self.cols.len() || self.guard.0 == 0)
    }

    fn obstacleloop(&mut self) -> usize {
        let mut iters = vec![];

        for i in 0..self.rows.len() {
            iters.push(
                (0..self.cols.len())
                    .into_par_iter()
                    .map(move |j| (i, j))
                    .filter(|(i, j)| {
                        let mut this = self.clone();
                        let mut visited = HashSet::new();

                        if (*i, *j) == this.guard {
                            return false;
                        }

                        // if obsticle at (i, j) continue
                        if this.rows[*i] & (u256::ONE) << j != 0 {
                            return false;
                        }

                        // add an obstacle at (i, j)
                        this.rows[*i] |= u256::ONE << j;
                        this.cols[*j] |= u256::ONE << i;

                        // run the simulation
                        while this.step() {
                            if !visited.insert((this.guard, this.direction)) {
                                return true;
                            }
                        }

                        false
                    }),
            );
        }

        let mut looping = 0;
        for iter in iters {
            looping += iter.count();
        }

        looping
    }
}

impl fmt::Display for Bitgame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputgame = Input::from(self.clone());

        write!(f, "{inputgame}")
    }
}

pub fn level1(mut game: Input) -> usize {
    game.run();
    game.visited()
}

pub fn level2(game: Input) -> usize {
    // we can store game with bitmaps (very efficient)
    let mut game = Bitgame::from(game);

    game.obstacleloop()
}
