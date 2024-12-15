#[path = "day15/level1.rs"]
mod level1_m;

#[path = "day15/level2.rs"]
mod level2_m;

use itertools::Itertools;
use libadvent::{
    grid::{Direction, Grid, GridParser, Point},
    AsInput, NewlineSeperated, Single,
};

use level1_m::Square as Square1;

pub struct Parser;
impl AsInput for Parser {
    type Input = (Grid<Square1>, Point, Vec<Direction>);

    fn from_str(s: &str) -> Self::Input {
        let parts = s.split("\n\n").collect_vec();

        let squares = GridParser::<Square1>::from_str(parts[0]);
        let moves = NewlineSeperated::<Single<Direction>>::from_str(parts[1]);
        let moves = moves.into_iter().flatten().rev().collect_vec();

        let robot = squares.find(&Square1::Robot);

        (squares, robot, moves)
    }
}

pub use level1_m::level1;
pub use level2_m::level2;
