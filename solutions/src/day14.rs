use itertools::Itertools;
use libadvent::{
    grid::{Offset, Point},
    AsInput, NewlineSeperated,
};

pub const MAX_W: isize = 101;
pub const MAX_H: isize = 103;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Robot {
    position: Point,
    velocity: Offset, // basically the same thing
}

impl Robot {
    pub fn step(&mut self, n: usize) {
        self.position += self.velocity * n;

        // wrap neg
        self.position.rem((MAX_H, MAX_W));
    }

    pub fn steponce(&mut self) {
        self.step(1);
    }

    pub fn quadrant(&self) -> Option<usize> {
        let halfw = MAX_W / 2;
        let halfh = MAX_H / 2;

        if self.position.x() < halfw {
            if self.position.y() < halfh {
                return Some(1);
            }

            if self.position.y() > halfh {
                return Some(2);
            }
        }

        if self.position.x() > halfw {
            if self.position.y() < halfh {
                return Some(3);
            }

            if self.position.y() > halfh {
                return Some(4);
            }
        }

        None
    }
}

impl AsInput for Robot {
    type Input = Self;

    fn from_str(s: &str) -> Self::Input {
        let lines = s.split(" ").collect_vec();
        let position = Point::parse_xy(&lines[0][2..], ",");
        let velocity = Offset::parse_xy(&lines[1][2..], ",");

        Self { position, velocity }
    }
}

pub type Parser = NewlineSeperated<Robot>;
type Input = <Parser as AsInput>::Input;

pub fn level1(data: Input) -> usize {
    data.into_iter()
        .filter_map(|mut robot| {
            robot.step(100);
            robot.quadrant()
        })
        .fold([0; 4], |mut acc, it| {
            acc[it - 1] += 1;
            acc
        })
        .into_iter()
        .product()
}

pub fn level2(mut data: Input) -> usize {
    let step = |data: &mut Input| data.iter_mut().for_each(Robot::steponce);
    let threshold = 60; // guess'd

    for i in 1.. {
        step(&mut data);

        let mut draw = [0u32; 100];

        for robot in &data {
            // basically blur
            let re_y = robot.position.y() / 11;
            let re_x = robot.position.x() / 11;
            let idx = (re_y * 10 + re_x) as usize;

            draw[idx] += 1;

            if draw[idx] >= threshold {
                return i;
            }
        }
    }

    0
}
