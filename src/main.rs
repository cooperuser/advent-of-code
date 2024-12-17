#![feature(slice_split_once)]
use clap::Parser;

use solution::Solution;

mod direction;
mod graph;
mod solution;
mod vector;
mod vector3;

mod aoc2023;
mod aoc2024;

#[derive(Parser, Debug)]
struct Args {
    /// Which year to select
    #[arg(short, long, default_value_t = 2024)]
    year: usize,
    /// Which day to run
    #[arg(short, long)]
    day: Option<usize>,
    /// Runs all days for the selected year
    #[arg(short, long)]
    all: bool,
    /// Benchmarks the selected day's solution
    #[arg(short, long)]
    bench: bool,
    /// How many iterations to average over in the benchmark
    #[arg(short, long, default_value_t = 10)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let years = &[aoc2023::DAYS, aoc2024::DAYS];
    let days = years[args.year - 2023];

    if args.all {
        for (index, day) in days.iter().enumerate() {
            println!("\n======= {} Day {:0>2} =======", args.year, index + 1);
            day(false);
        }
        return;
    }

    let day = args.day.unwrap_or(days.len());
    if args.bench {
        let count = args.count as u32;
        let times: Vec<_> = (0..count).map(|_| days[day - 1](true).unwrap()).collect();
        let parse = times.iter().map(|(p, _, _)| p).sum::<std::time::Duration>() / count;
        let a = times.iter().map(|(_, a, _)| a).sum::<std::time::Duration>() / count;
        let b = times.iter().map(|(_, _, b)| b).sum::<std::time::Duration>() / count;
        println!("======= {} Day {day:0>2} =======", args.year);
        println!("(average over {count} runs)");
        println!("parse :\t{:?}", parse);
        println!("part_a:\t{:?}", a);
        println!("part_b:\t{:?}", b);
        return;
    }

    println!("===== {} Day {day:0>2} =====", args.year);
    days[day - 1](false);
}

pub fn split(input: String) -> Vec<String> {
    input
        .trim_end()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}
