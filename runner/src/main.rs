use libadvent::Solution;
use solutions::*;

macro_rules! runner {
    ($($day:ident),*) => {
        let mut iter = 1..;

        println!("Pick a day:");
        $(
            println!("{}) {}", iter.next().unwrap(), concat!(stringify!($day), "a"));
            println!("{}) {}", iter.next().unwrap(), concat!(stringify!($day), "b"));
        )*

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<usize>().unwrap();

        let mut iter = 1..;

        match input {
            $(
                i if i == iter.next().unwrap() => {
                    $day::_Solution::run(
                        include_str!(
                            concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/../dataset/",
                                stringify!($day),
                                ".txt"
                            )
                        ),
                        1
                    )
                },
                i if i == iter.next().unwrap() => {
                    $day::_Solution::run(
                        include_str!(
                            concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/../dataset/",
                                stringify!($day),
                                ".txt"
                            )
                        ),
                        2
                    )
                },
            )*
            _ => println!("Invalid day"),
        }
    };
}

fn main() {
    runner!(day01, day02, day03);
}
