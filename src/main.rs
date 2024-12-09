#![feature(slice_split_once)]

use solution::Solution;

mod direction;
mod solution;
mod vector;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let days = &[
        day01::Day::run,
        day02::Day::run,
        day03::Day::run,
        day04::Day::run,
        day05::Day::run,
        day06::Day::run,
        day07::Day::run,
        day08::Day::run,
        day09::Day::run,
    ];

    match args.get(1) {
        Some(arg) if arg == "all" => {
            for (index, day) in days.iter().enumerate() {
                println!("\n===== Day {} =====", index + 1);
                day(false);
            }
        }
        Some(arg) => {
            let Ok(day) = arg.parse::<usize>() else {
                return;
            };
            match args.get(2) {
                Some(arg) if arg == "--time" => {
                    let times: Vec<_> = (0..10).map(|_| days[day - 1](true).unwrap()).collect();
                    let parse = times.iter().map(|(p, _, _)| p).sum::<std::time::Duration>() / 10;
                    let a = times.iter().map(|(_, a, _)| a).sum::<std::time::Duration>() / 10;
                    let b = times.iter().map(|(_, _, b)| b).sum::<std::time::Duration>() / 10;
                    println!("======= Day {day:0>2} =======");
                    println!("(average over 10 runs)");
                    println!("parse :\t{:?}", parse);
                    println!("part_a:\t{:?}", a);
                    println!("part_b:\t{:?}", b);
                }
                _ => {
                    println!("===== Day {day:0>2} =====");
                    days[day - 1](false);
                }
            }
        }
        None => {
            println!("===== Day {} =====", days.len());
            days.last().unwrap()(false);
        }
    }
}

pub fn split(input: String) -> Vec<String> {
    input
        .trim_end()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}
