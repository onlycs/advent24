use libadvent::AsInput;

use paste::paste;
use solutions::*;

macro_rules! runner {
    (run $day:ident $level:expr) => {
        paste! {
            $day::[<level $level>](
                $day::Parser::from_str(
                    include_str!(
                        concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../dataset/",
                            stringify!($day),
                            ".txt"
                        )
                    )
                )
            )
        }
    };

    ($($day:ident),*) => {
        let mut iter = 1..;

        println!("Pick a day:");
        $(
            let i = iter.next().unwrap();

            println!("{}a) {}", i, concat!(stringify!($day), "a"));
            println!("{}b) {}", i, concat!(stringify!($day), "b"));
        )*

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut iter = (1..).map(|i| (i + 1) / 2);

        match input {
            $(
                i if i.trim() == format!("{}a", iter.next().unwrap()) => {
                    println!("{}", runner!(run $day 1));
                },
                i if i.trim() == format!("{}b", iter.next().unwrap()) => {
                    println!("{}", runner!(run $day 2));
                },
            )*
            _ => println!("Invalid day"),
        }
    };
}

fn main() {
    runner!(day01, day02, day03, day04, day05, day06, day07, day08, day09, day10);
}
