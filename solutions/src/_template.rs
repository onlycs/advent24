use libadvent::IsInput;

pub struct Input;

impl IsInput for Input {
    fn parse(s: &str) -> Self {
        todo!()
    }
}

problem_parser!(ty Input);

pub fn level1(_: Input) -> i32 {
    0
}

pub fn level2(_: Input) -> i32 {
    0
}
