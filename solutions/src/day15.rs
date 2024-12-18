#[path = "day15/level1.rs"]
mod level1_m;

#[path = "day15/level2.rs"]
mod level2_m;

use itertools::Itertools;
use libadvent::{
    grid::{Direction, Grid, GridParser, Point},
    Parser, Seperated, Take,
};

use level1_m::Square as Square1;

pub struct InputParser;
impl Parser for InputParser {
    type Output = (Grid<Square1>, Point, Vec<Direction>);

    fn parse(&mut self, s: &str) -> Self::Output {
        let parts = s.split("\n\n").collect_vec();

        let squares = GridParser::new(ty_parser!(Square1)).parse(parts[0]);
        let moves = Seperated::newline(Take::one(ty_parser!(Direction))).parse(parts[1]);
        let moves = moves.into_iter().flatten().rev().collect_vec();

        let robot = squares.find(&Square1::Robot);

        (squares, robot, moves)
    }
}

problem_parser!(parser InputParser);

pub use level1_m::level1;
pub use level2_m::level2;
