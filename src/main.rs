#![feature(slice_split_once)]

use solution::{Output, Solution};

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
    let day = day::<day09::Day, i64>();
    println!("parse :\t{:?}\n", day.duration);

    match day.sample_a.part_a() {
        Some(received) if received == day.answer_a => {
            let start = std::time::Instant::now();
            let answer = day.real.part_a();
            let duration = start.elapsed();
            println!("part_a:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        }
        Some(received) => {
            println!("part_a: failed!");
            println!("\texpected: {}", day.answer_a);
            println!("\treceived: {}", received);
        }
        None => {
            println!("part_a: unsolved!");
        }
    }

    match day.sample_b.part_b() {
        Some(received) if received == day.answer_b => {
            let start = std::time::Instant::now();
            let answer = day.real.part_b();
            let duration = start.elapsed();
            println!("part_b:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        }
        Some(received) => {
            println!("part_b: failed!");
            println!("\texpected: {}", day.answer_b);
            println!("\treceived: {}", received);
        }
        None => {
            println!("part_b: unsolved!");
        }
    }
}

struct Day<D: Solution<T>, T: Output> {
    real: D,
    sample_a: D,
    sample_b: D,
    answer_a: T,
    answer_b: T,
    duration: std::time::Duration,
}

fn day<D: Solution<T>, T: Output>() -> Day<D, T> {
    let meta = D::meta();
    let sample_a = D::new(split(meta.sample_a));
    let sample_b = D::new(split(meta.sample_b));
    let start = std::time::Instant::now();
    let real = D::new(split(meta.input));
    let duration = start.elapsed();
    Day::<D, T> {
        real,
        sample_a,
        sample_b,
        answer_a: meta.answer_a,
        answer_b: meta.answer_b,
        duration,
    }
}

pub fn split(input: String) -> Vec<String> {
    input
        .trim_end()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}
