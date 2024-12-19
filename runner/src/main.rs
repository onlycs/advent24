use libadvent::Parser;
use reqwest::StatusCode;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

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
            input(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../dataset/",
                stringify!($day),
                ".txt"
            )).as_str()
        )
    };

    ($($day:ident),*) => {
        println!("╭─ Pick a day");
        println!("│");
        println!("├─ Format: DD[a|b]");
        println!("├─ Example: 01a");
        println!("├─ Or enter \"fetch\" to fetch all inputs");
        println!("│");
        print!("\n\n\n\n\n\x1b[5A╰─ ");

        let mut input = String::new();
        std::io::stdout().lock().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "fetch" {
            println!("\x1b[1A╰─ {}", "Fetching...".yellow());
            fetch();
            return;
        }

        let mut iter = (1..).map(|i| (i + 1) / 2);

        println!("\x1b[1A╰─ {}", "Waiting...".yellow());
        std::io::stdout().lock().flush().unwrap();

        let t2parse;
        let t2run;

        match input {
            $(
                i if i.trim() == format!("{:02}a", iter.next().unwrap()) => {
                    let timer = ::std::time::Instant::now();
                    let parsed = runner!(parse $day);
                    t2parse = timer.elapsed();

                    let timer = ::std::time::Instant::now();
                    let output = runner!(run $day 1 parsed);
                    t2run = timer.elapsed();

                    print!("\x1b[1A╰─ ");
                    println!("{} - {}", stringify!($day).blue(), "Level 1".magenta());
                    println!("\toutput:\t{}", output.to_string().cyan());
                },
                i if i.trim() == format!("{:02}b", iter.next().unwrap()) => {
                    let timer = ::std::time::Instant::now();
                    let parsed = runner!(parse $day);
                    t2parse = timer.elapsed();

                    let timer = ::std::time::Instant::now();
                    let output = runner!(run $day 2 parsed);
                    t2run = timer.elapsed();

                    print!("\x1b[1A╰─ ");
                    println!("{} - {}", stringify!($day).blue(), "Level 2".magenta());
                    println!("\toutput:\t{}", output.to_string().cyan());
                },
            )*
            _ => {
                println!("\x1b[1A╰─ {}", "Invalid day".red());
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

fn input(file: &'static str) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file)
        .expect("Could not open file");

    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Could not read file");

    input.trim().to_string()
}

fn fetch() {
    // try to open .token.txt
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(concat!(env!("CARGO_MANIFEST_DIR"), "/../.token.txt"))
        .expect("Could not open .token.txt");

    // check if file empty
    let mut token = String::new();

    file.read_to_string(&mut token)
        .expect("Could not read .token.txt");

    if token.is_empty() {
        println!("\x1b[1A├─ Enter your token");
        print!("╰─ ");

        std::io::stdout().lock().flush().unwrap();
        std::io::stdin().read_line(&mut token).unwrap();

        file.write_all(token.as_bytes())
            .expect("Could not write to .token.txt");

        println!(
            "\x1b[2A╰─ {}     \n{}  \n\x1b[2A",
            "Fetching...".yellow(),
            " ".repeat(token.len())
        );
    }

    let token = token.trim();

    for day in 1..=25 {
        // if file exists, skip
        if Path::new(&format!(
            "{}/../dataset/day{:02}.txt",
            env!("CARGO_MANIFEST_DIR"),
            day
        ))
        .exists()
        {
            println!("\x1b[1A╰─ {}", format!("Skipping Day {day:02}...").green());
            continue;
        }

        // attempt to fetch the token using reqwest
        println!("\x1b[1A╰─ {}", format!("Fetching Day {day:02}...").yellow());

        let url = format!("https://adventofcode.com/2024/day/{}/input", day);
        let cookie = format!("session={}", token);

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(&url)
            .header("cookie", cookie)
            .send()
            .expect("Failed to fetch input");

        match response.status() {
            StatusCode::NOT_FOUND => break,
            StatusCode::INTERNAL_SERVER_ERROR => {
                println!("\x1b[2A╰─ {}", "Invalid Token".red());
                break;
            }
            _ => {}
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!(
                "{}/../dataset/day{:02}.txt",
                env!("CARGO_MANIFEST_DIR"),
                day
            ))
            .expect("Could not open file");

        file.write_all(&response.bytes().expect("Failed to read response"))
            .unwrap();

        file.flush().unwrap();
    }

    println!("\x1b[1A╰─ {}", "Fetched               ".green());
}

fn main() {
    runner!(
        day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
        day14, day15, day16, day17, day18, day19
    );
}
