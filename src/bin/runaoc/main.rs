use std::{fmt::Display, time::Instant};

mod aoc2022;
use aoc2022::*;

const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = 2022;

const FIRST_DAY: usize = 1;
const LAST_DAY: usize = 25;
pub enum Selector {
    All,
    One(usize),
    Last,
    //Range(usize, usize),
}

pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn parse(&mut self) -> ();
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

fn main() -> () {
    // parse cli args

    let years: Vec<fn(Selector)> = vec![
        run_2022, // 2015
        run_2022, //2016
        run_2022, //2017
        run_2022, //2018
        run_2022, //2019
        run_2022, //2020
        run_2022, //2021
        run_2022, //2022
    ];

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 && args[1] == "all".to_string() {
        for year in years {
            year(Selector::All);
        }
    } else if args.len() == 3 {
        let year = if let Ok(year) = args[1].parse::<usize>() {
            year
        } else {
            eprint!("Error parsing year: Invalid year {}", args[1]);
            std::process::exit(1);
        };

        if !(FIRST_YEAR..=LAST_YEAR).contains(&year) {
            eprint!(
                "Error parsing year: You submitted: {}. Year must be in the range {} to {}. ",
                args[1], FIRST_YEAR, LAST_YEAR
            );
            std::process::exit(1);
        }

        if args[2] == "all".to_string() {
            years[year - 2015](Selector::All);
        } else {
            let day = if let Ok(day) = args[2].parse::<usize>() {
                day
            } else {
                eprint!("Error parsing day: Invalid day {}", args[1]);
                std::process::exit(1);
            };

            if !(FIRST_DAY..=LAST_DAY).contains(&day) {
                eprint!(
                    "Error parsing day: You submitted: {}. Day must be in the range {} to {}. ",
                    args[1], FIRST_DAY, LAST_DAY
                );
                std::process::exit(1);
            }

            years[year - 2015](Selector::One(day));
        }
    } else {
        println!("====Running Last====");
        years[years.len() - 1](Selector::Last)
    }
}

pub fn output<T: Display>(out: T) -> Vec<String> {
    vec![format!("{}", out)]
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T) -> () {
    let name = solution.name();
    println!("---- {} Day {} ----", name.0, name.1);

    let start = Instant::now();
    solution.parse();
    let elapsed = start.elapsed().as_millis();
    println!("{} Parsing...", format_time(elapsed));

    let start = Instant::now();
    let p1 = solution.part1();
    let elapsed = start.elapsed().as_millis();
    print_output(1, p1, elapsed);

    let start = Instant::now();
    let p2 = solution.part2();
    let elapsed = start.elapsed().as_millis();
    print_output(2, p2, elapsed);
}

fn print_output(which: u8, sol: Vec<String>, time: u128) -> () {
    let mut ans = sol.iter();
    // unwrap is fine here there must always be one element otherwise output has failed
    println!(
        "{} Part {}: {}",
        format_time(time),
        which,
        ans.next().unwrap()
    );

    for line in ans {
        println!("{:15}{}", " ", line);
    }
}

fn format_time(time: u128) -> String {
    format!("{}.{:03}s", time / 1000, time % 1000)
}
