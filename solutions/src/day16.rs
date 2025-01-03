use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use libadvent::{
    grid::{Direction, Grid, GridParser, Point},
    IsInput, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Wall,
}

impl IsInput for Square {
    fn parse(s: &str) -> Self {
        match s {
            "." | "S" | "E" => Self::Empty,
            "#" => Self::Wall,
            _ => panic!("Invalid square: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maze {
    pub grid: Grid<Square>,
    pub src: Point,
    pub dest: Point,
}

impl Maze {
    fn dijkstra(&self) -> usize {
        let mut dist = self.grid.map(|_, _| usize::MAX);
        let mut heap = BinaryHeap::new();
        let mut min_cost = usize::MAX;

        dist[self.src] = 0;
        heap.push(Reverse((0usize, (self.src, Direction::Right))));

        while let Some(Reverse((cost, (pos, from)))) = heap.pop() {
            // the shortest path may not show up in the fewest iterations
            if pos == self.dest && cost < min_cost {
                min_cost = cost;
                continue;
            }

            // no need to explore further if the cost is already higher
            if cost > dist[pos] || cost >= min_cost {
                continue;
            }

            for dir in Direction::ALL {
                let next = pos + *dir;
                let cost = cost + if dir == from { 1 } else { 1001 };

                if self.grid[next] == Square::Wall {
                    continue;
                }

                if cost < dist[next] {
                    dist[next] = cost;
                    heap.push(Reverse((cost, (next, dir))));
                }
            }
        }

        min_cost
    }

    fn dijkstra_uniq_squares(&self) -> usize {
        let min_cost = self.dijkstra();
        let mut dist = self.grid.map(|_, _| [min_cost, min_cost]);

        // (cost, (pos, dir, hist))
        let mut heap = Vec::new();
        let mut points = HashSet::new();

        dist[self.src][Direction::Right.axis_ord()] = 0;
        heap.push((0usize, (self.src, Direction::Right, vec![])));

        while let Some((cost, (pos, from, mut hist))) = heap.pop() {
            // backtracing
            hist.push(pos);

            // no need to explore further if the cost is already higher
            if cost > dist[pos][from.axis_ord()] || cost > min_cost {
                continue;
            }

            // the shortest path may not show up in the fewest iterations
            if pos == self.dest {
                if cost == min_cost {
                    points.extend(hist);
                }

                continue;
            }

            for dir in Direction::ALL {
                let next = if dir == from { pos + *dir } else { pos };
                let cost = cost + if dir == from { 1 } else { 1000 };

                if self.grid[next] == Square::Wall {
                    continue;
                }

                if cost <= dist[next][dir.axis_ord()] {
                    dist[next][dir.axis_ord()] = cost;
                    heap.push((cost, (next, dir, hist.clone())));
                }
            }
        }

        points.len()
    }
}

impl IsInput for Maze {
    fn parse(s: &str) -> Self {
        let mut src = Point::ORIGIN;
        let mut dest = Point::ORIGIN;

        let grid = GridParser::with_f(ty_parser!(Square), |ch, p| match ch {
            'S' => src = p,
            'E' => dest = p,
            _ => {}
        })
        .parse(s);

        Self { grid, src, dest }
    }
}

problem_parser!(ty Maze);

pub fn level1(maze: Maze) -> usize {
    maze.dijkstra()
}

pub fn level2(maze: Maze) -> usize {
    maze.dijkstra_uniq_squares()
}
