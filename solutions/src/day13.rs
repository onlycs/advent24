//use ethnum::{Asusize, usize};
use itertools::Itertools;
use libadvent::{
    grid::{Offset, Point},
    AsInput, CharSeperated,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClawMachine {
    a: Offset,
    b: Offset,
    r: Point,
}

impl ClawMachine {
    fn add_1e13(self) -> Self {
        ClawMachine {
            r: self.r + Offset(1e13 as isize, 1e13 as isize),
            ..self
        }
    }

    #[allow(non_snake_case)] // linear algebra
    fn solve(self) -> Option<usize> {
        // Pa * a + Pb * b = r
        //
        // Pa * Ax + Pb * Bx = Rx
        // Pa * Ay + Pb * By = Ry
        //
        // Thanks, Cramer
        // and 3Blue1Brown for helping me understand wtfever this is

        let Offset(Ay, Ax) = self.a;
        let Offset(By, Bx) = self.b;
        let Point(Ry, Rx) = self.r;

        // Pa = Dx / D
        // Pb can be solved later

        // D = det (| Ay By |) = Ay * Bx - By * Ax
        //         (| Ax Bx |)
        let D = Ay * Bx - By * Ax;

        // Dx = det (| Bx By |) = Bx * Ry - By * Rx
        //          (| Rx Ry |)
        let Dx = Bx * Ry - By * Rx;

        // check for errors (i.e. Dx and Dy are not multiples of D)
        if Dx % D != 0 {
            return None;
        }

        // Pa = Dx / D
        let Pa = Dx / D;

        // check for errors v2
        if (Rx - Pa * Ax) % Bx != 0 {
            return None;
        }

        // therefore, Pb = (Rx - Pa * Ax) / Bx
        let Pb = (Rx - Pa * Ax) / Bx;

        // b costs 1 point, a costs 3
        let apts = Pa * 3;
        let bpts = Pb;

        Some((apts + bpts) as usize)
    }
}

impl AsInput for ClawMachine {
    type Input = Self;

    fn from_str(s: &str) -> Self::Input {
        let lines = s.lines().collect_vec();

        let parsebtn = |s: &str| {
            let data = s.split(": ").collect_vec()[1];
            let parts = data.split(", ").collect_vec();

            let xoff = parts[0][1..].parse().unwrap();
            let yoff = parts[1][1..].parse().unwrap();

            Offset(yoff, xoff)
        };

        let parseprize = |s: &str| {
            let data = s.split(": ").collect_vec()[1];
            let parts = data.split(", ").collect_vec();
            let x = parts[0][2..].parse().unwrap();
            let y = parts[1][2..].parse().unwrap();

            Point(y, x)
        };

        let btna = parsebtn(lines[0]);
        let btnb = parsebtn(lines[1]);
        let prize = parseprize(lines[2]);

        ClawMachine {
            a: btna,
            b: btnb,
            r: prize,
        }
    }
}

pub type Parser = CharSeperated<ClawMachine, '\n', 2>;
type Input = <Parser as AsInput>::Input;

pub fn level1(data: Input) -> usize {
    data.into_par_iter()
        .filter_map(ClawMachine::solve)
        .sum::<usize>()
}

pub fn level2(data: Input) -> usize {
    data.into_par_iter()
        .filter_map(|cm| cm.add_1e13().solve())
        .sum::<usize>()
}
