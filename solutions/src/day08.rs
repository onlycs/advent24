use core::fmt;
use std::{ops::Index, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Square {
    tower: Option<char>,
    antinodes: Vec<char>,
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .map(|c| Square {
                tower: if c == '.' { None } else { Some(c) },
                antinodes: vec![],
            })
            .ok_or_else(|| "Empty string".to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    squares: Vec<Vec<Square>>,
    chars: Vec<char>,
}

impl Input {
    pub fn get(&self, c: char) -> Vec<(usize, usize)> {
        self.squares
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, square)| {
                        if square.tower == Some(c) {
                            Some((i, j))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    pub fn antinodify(&mut self, c: char) {
        // antinodes are all points that are colinear with two towers and twice the distance
        // from one tower as the other
        //
        // find all antinodes for towers of type c

        let towers = self.get(c);
        let perms = towers.iter().permutations(2);

        for perm in perms {
            let [towa, towb] = perm.as_slice() else {
                continue;
            };

            let (ia, ja) = **towa;
            let (ib, jb) = **towb;

            // cast to i32
            let (ia, ja, ib, jb) = (ia as i32, ja as i32, ib as i32, jb as i32);

            // if two points are equal
            if ia == ib && ja == jb {
                continue;
            }

            // ia + n = ib, ib + n = io
            let io = ia + 2 * (ib - ia);
            let jo = ja + 2 * (jb - ja);

            // ib - n = ia, ia - n = io2
            let io2 = ib - 2 * (ib - ia);
            let jo2 = jb - 2 * (jb - ja);

            // check (io, jo) inbounds
            if io >= 0
                && io < self.squares.len() as i32
                && jo >= 0
                && jo < self.squares[0].len() as i32
            {
                let (io, jo) = (io as usize, jo as usize);
                let square = &mut self.squares[io][jo];
                square.antinodes.push(c);
            }

            // check (io2, jo2) inbounds
            if io2 >= 0
                && io2 < self.squares.len() as i32
                && jo2 >= 0
                && jo2 < self.squares[0].len() as i32
            {
                let (io2, jo2) = (io2 as usize, jo2 as usize);
                let square = &mut self.squares[io2][jo2];
                square.antinodes.push(c);
            }
        }
    }

    pub fn antinodify_2(&mut self, c: char) {
        // all points that are colinear with two towers, regardless of distance

        let towers = self.get(c);
        let perms = towers.iter().permutations(2);

        for perm in perms {
            let [towa, towb] = perm.as_slice() else {
                continue;
            };

            let (ia, ja) = **towa;
            let (ib, jb) = **towb;

            // cast to i32
            let (mut ia, mut ja, mut ib, mut jb) = (ia as i32, ja as i32, ib as i32, jb as i32);

            // if two points are equal
            if ia == ib && ja == jb {
                continue;
            }

            // ia + n = ib, ib + n = io
            let di = ib - ia;
            let dj = jb - ja;

            while ia + di >= 0
                && ia + di < self.squares.len() as i32
                && ja + dj >= 0
                && ja + dj < self.squares[0].len() as i32
            {
                let (io, jo) = (ia + di, ja + dj);
                let square = &mut self.squares[io as usize][jo as usize];
                square.antinodes.push(c);

                ia += di;
                ja += dj;
            }

            while ib - di >= 0
                && ib - di < self.squares.len() as i32
                && jb - dj >= 0
                && jb - dj < self.squares[0].len() as i32
            {
                let (io, jo) = (ib - di, jb - dj);
                let square = &mut self.squares[io as usize][jo as usize];
                square.antinodes.push(c);

                ib -= di;
                jb -= dj;
            }
        }
    }

    pub fn antinodify_all(&mut self) {
        for c in self.chars.clone().iter() {
            self.antinodify(*c);
        }
    }

    pub fn antinodify_all2(&mut self) {
        for c in self.chars.clone().iter() {
            self.antinodify_2(*c);
        }
    }

    pub fn count_antinodes(&self) -> usize {
        self.squares
            .iter()
            .flat_map(|row| row.iter())
            .filter(|square| !square.antinodes.is_empty())
            .count()
    }
}

impl Index<(usize, usize)> for Input {
    type Output = Square;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.squares[i][j]
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.squares {
            for square in row {
                if let Some(tower) = square.tower {
                    if !square.antinodes.is_empty() {
                        // write in red
                        write!(f, "\x1b[31m{}\x1b[0m", tower)?;
                    } else {
                        write!(f, "{}", tower)?;
                    }
                } else if !square.antinodes.is_empty() {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let squares = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Square>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let chars = squares
            .iter()
            .flat_map(|row| row.iter().filter_map(|square| square.tower))
            .collect_vec();

        Ok(Input { squares, chars })
    }
}

pub fn level1(mut input: Input) -> usize {
    input.antinodify_all();

    println!("{input}");

    input.count_antinodes()
}

pub fn level2(mut input: Input) -> usize {
    input.antinodify_all2();

    println!("{input}");

    input.count_antinodes()
}
