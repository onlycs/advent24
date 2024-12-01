use std::{fmt::Debug, str::FromStr};

pub trait AsInput {
    type Input;

    fn from_str(s: &str) -> Self::Input;
}

pub trait Solution {
    type Input: AsInput;
    type Output1: ToString;
    type Output2: ToString;

    fn day1(input: <Self::Input as AsInput>::Input) -> Self::Output1;
    fn day2(input: <Self::Input as AsInput>::Input) -> Self::Output2;

    fn run(input: &'static str, _part: u8) {
        let input = Self::Input::from_str(input);

        match _part {
            1 => {
                let output = Self::day1(input);
                println!("{}", output.to_string());
            }
            2 => {
                let output = Self::day2(input);
                println!("{}", output.to_string());
            }
            _ => println!("Invalid part"),
        }
    }
}

pub struct CommaSeperated<T>(std::marker::PhantomData<T>);
pub struct WhiteSeperated<T>(std::marker::PhantomData<T>);
pub struct NewlineSeperated<T>(std::marker::PhantomData<T>);

impl<T: AsInput> AsInput for CommaSeperated<T> {
    type Input = Vec<T::Input>;

    fn from_str(s: &str) -> Self::Input {
        s.split(',').map(T::from_str).collect()
    }
}

impl<T: AsInput> AsInput for WhiteSeperated<T> {
    type Input = Vec<T::Input>;

    fn from_str(s: &str) -> Self::Input {
        s.split_whitespace().map(T::from_str).collect()
    }
}

impl<T: AsInput> AsInput for NewlineSeperated<T> {
    type Input = Vec<T::Input>;

    fn from_str(s: &str) -> Self::Input {
        s.lines().map(T::from_str).collect()
    }
}

impl<T: FromStr> AsInput for T
where
    <T as FromStr>::Err: Debug,
{
    type Input = T;

    fn from_str(s: &str) -> Self::Input {
        s.parse().unwrap()
    }
}
