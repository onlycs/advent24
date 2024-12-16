use std::time::Duration;

use libadvent::Parser;

use colored::Colorize;
use paste::paste;
use solutions::*;

macro_rules! runner {
    (run $day:ident $level:tt $parsed:ident) => {
        paste! {
            $day::[<level $level>]($parsed)
        }
    };

    (parse $day:ident) => {
        $day::parser().parse(
            include_str!(
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../dataset/",
                    stringify!($day),
                    ".txt"
                )
            )
        )
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

        let t2parse;
        let t2run;

        match input {
            $(
                i if i.trim() == format!("{}a", iter.next().unwrap()) => {
                    let timer = ::std::time::Instant::now();
                    let parsed = runner!(parse $day);
                    t2parse = timer.elapsed();

                    let timer = ::std::time::Instant::now();
                    let output = runner!(run $day 1 parsed);
                    t2run = timer.elapsed();

                    println!("{} - {}", stringify!($day).blue(), "Level 1".magenta());
                    println!("\t{}:\t{}", "output".cyan(), output);
                },
                i if i.trim() == format!("{}b", iter.next().unwrap()) => {
                    let timer = ::std::time::Instant::now();
                    let parsed = runner!(parse $day);
                    t2parse = timer.elapsed();

                    let timer = ::std::time::Instant::now();
                    let output = runner!(run $day 2 parsed);
                    t2run = timer.elapsed();

                    println!("{} - {}", stringify!($day).blue(), "Level 2".magenta());
                    println!("\toutput:\t{}", output.to_string().cyan());
                },
            )*
            _ => {
                println!("Invalid day");
                ::std::process::exit(1);
            },
        }

        println!("\tparse:\t{}", colorize(t2parse, 4, 8));
        println!("\trun:\t{}", colorize(t2run, 75, 200));
        println!("\ttotal:\t{}", colorize(t2parse + t2run, 80, 210));
    };
}

fn colorize(time: Duration, a: u64, b: u64) -> String {
    let disp = format!("{time:?}");

    if time < Duration::from_millis(a) {
        format!("{}", disp.green())
    } else if time < Duration::from_millis(b) {
        format!("{}", disp.yellow())
    } else {
        format!("{}", disp.red())
    }
}

fn main() {
    runner!(
        day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
        day14, day15
    );
}
