mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

fn main() {
    use day18::*;

    let sample_a = Solution::new(split(SAMPLE_A));
    let sample_b = Solution::new(split(SAMPLE_B));
    let start = std::time::Instant::now();
    let real = Solution::new(split(INPUT));
    let duration = start.elapsed();
    println!("parse :\t{:?}\n", duration);

    match sample_a.part_a() {
        Some(ANSWER_A) => {
            let start = std::time::Instant::now();
            let answer = real.part_a();
            let duration = start.elapsed();
            println!("part_a:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        },
        Some(received) => {
            println!("part_a: failed!");
            println!("\texpected: {}", ANSWER_A);
            println!("\treceived: {}", received);
        },
        None => {
            println!("part_a: unsolved!");
        }
    }

    match sample_b.part_b() {
        Some(ANSWER_B) => {
            let start = std::time::Instant::now();
            let answer = real.part_b();
            let duration = start.elapsed();
            println!("part_b:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        },
        Some(received) => {
            println!("part_b: failed!");
            println!("\texpected: {}", ANSWER_B);
            println!("\treceived: {}", received);
        },
        None => {
            println!("part_b: unsolved!");
        }
    }
}

pub fn split(input: &str) -> Vec<String> {
    input.trim_end().split('\n').map(|s| s.trim().to_string()).collect()
}
