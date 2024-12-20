use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use libadvent::{
    grid::{Direction, Grid, GridParser, Point},
    IsInput, Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Wall,
    Empty,
}

impl IsInput for Square {
    fn parse(s: &str) -> Self {
        match s {
            "#" => Self::Wall,
            "." | "S" | "E" => Self::Empty,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct State {
    point: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Square>,
    src: Point,
    dest: Point,
}

impl Input {
    pub fn dijkstra(&self, hack_steps: usize) -> usize {
        let mut dist = self.grid.map(|_, _| usize::MAX);
        let mut heap = BinaryHeap::new();

        dist[self.src] = 0;
        heap.push(State {
            point: self.src,
            cost: 0,
        });

        while let Some(State { point, cost, .. }) = heap.pop() {
            if point == self.dest {
                continue;
            }

            if cost > dist[point] {
                continue;
            }

            for dir in Direction::ALL {
                let next = point + *dir;
                let cost = cost + 1;

                if !self.grid.inbounds(next) || self.grid[next] == Square::Wall {
                    continue;
                }

                if cost < dist[next] {
                    dist[next] = cost;
                    heap.push(State { point: next, cost });
                }
            }
        }

        // collect dist into hmap because it's faster for some reason? idk.
        let dist = dist
            .into_iter()
            .filter(|(_, n)| *n != usize::MAX)
            .collect::<HashMap<_, _>>();

        let mut better = 0;

        for ((pt1, cost1), (pt2, cost2)) in dist.iter().tuple_combinations() {
            // equal to the steps we take to get there because we can only move orthogonally
            let dist = pt1.manhattan(*pt2);

            // re-evaluate cost. if we can cut a direct path between pt1 and pt2, we can save all of the steps
            // to get there normally (but add the distance, because hacking still costs the same amount)
            let diff = cost1.abs_diff(*cost2) - dist;

            // if we cannot join the paths, we can't save any steps
            if dist > hack_steps {
                continue;
            }

            // we need to save at least 100 steps for it to count towards the answer
            if diff >= 100 {
                better += 1;
            }
        }

        better
    }
}

impl IsInput for Input {
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

problem_parser!(ty Input);

pub fn level1(input: Input) -> usize {
    input.dijkstra(2)
}

pub fn level2(input: Input) -> usize {
    input.dijkstra(20)
}
