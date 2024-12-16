use libadvent::{
    grid::{Grid, GridParser, Point},
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

// pub struct Parser;

// type Input = <Parser as Parser>::Input;

// pub fn level1(_: Input) -> i32 {
//     0
// }

// pub fn level2(_: Input) -> i32 {
//     0
// }
