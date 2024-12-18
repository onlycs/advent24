use std::{cmp, collections::BinaryHeap, usize};

use libadvent::{
    grid::{Direction, Grid, Point, PointParser},
    Reverse, Seperated,
};

problem_parser!(Seperated::newline(PointParser::new(Reverse::new(
    Seperated::comma(ty_parser!(usize))
))));

type Input = Vec<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    coord: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Grid<bool>) -> usize {
    let src = Point::new(0, 0);
    let dest = Point::new(70, 70);

    let mut dist = grid.map(|_, _| usize::MAX);
    let mut heap = BinaryHeap::new();

    dist[src] = 0;
    heap.push(State {
        cost: 0,
        coord: src,
    });

    while let Some(State { cost, coord }) = heap.pop() {
        if coord == dest {
            return cost;
        }

        if cost > dist[coord] {
            continue;
        }

        for dir in Direction::ALL {
            let next = coord + *dir;
            let cost = cost + 1;

            if (!grid.inbounds(next)) || grid[next] {
                continue;
            }

            if cost < dist[next] {
                dist[next] = cost;
                heap.push(State { cost, coord: next });
            }
        }
    }

    usize::MAX
}

pub fn level1(points: Input) -> usize {
    let mut grid = Grid::new(false, 71, 71);

    for point in &points[0..1024] {
        grid[*point] = true;
    }

    dijkstra(&grid)
}

pub fn level2(points: Input) -> String {
    let mut lb = 0;
    let mut ub = points.len();

    loop {
        // bisect
        let mid = (lb + ub) / 2;
        let mut grid = Grid::new(false, 71, 71);

        for point in &points[0..mid] {
            grid[*point] = true;
        }

        let cost = dijkstra(&grid);

        if cost == usize::MAX {
            ub = mid;
        } else {
            lb = mid;
        }

        if ub - lb == 1 {
            break;
        }
    }

    format!("{},{}", points[lb].1, points[lb].0)
}
