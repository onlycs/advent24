use std::{fmt::Debug, str::FromStr};

pub trait AsInput {
    type Input;

    fn from_str(s: &str) -> Self::Input;
}

pub struct CommaSeperated<T>(std::marker::PhantomData<T>);
pub struct WhiteSeperated<T>(std::marker::PhantomData<T>);
pub struct NewlineSeperated<T>(std::marker::PhantomData<T>);
pub struct Single<T>(std::marker::PhantomData<T>);

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

impl<T: AsInput> AsInput for Single<T> {
    type Input = Vec<T::Input>;

    fn from_str(s: &str) -> Self::Input {
        s.chars()
            .map(|c| c.to_string())
            .map(|s| T::from_str(&s))
            .collect()
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
