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
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day25;

fn main() {
    use day25::*;

    let sample = Solution::new(split(SAMPLE));
    let real = Solution::new(split(INPUT));

    let part_a = sample.part_a();
    if part_a == SAMPLE_A {
        let start = std::time::Instant::now();
        println!("part_a:\t{}", real.part_a());
        let duration = start.elapsed();
        println!(" ** in:\t{:?}", duration);
    } else {
        println!("part_a: failed!");
        println!("\texpected: {}", SAMPLE_A);
        println!("\treceived: {}", part_a);
    }

    let part_b = sample.part_b();
    if part_b == SAMPLE_B {
        let start = std::time::Instant::now();
        println!("part_b:\t{}", real.part_b());
        let duration = start.elapsed();
        println!(" ** in:\t{:?}", duration);
    } else {
        println!("part_b: failed!");
        println!("\texpected: {}", SAMPLE_B);
        println!("\treceived: {}", part_b);
    }
}

pub fn split(input: &str) -> Vec<String> {
    input.trim_end().split("\n").map(|s| s.to_string()).collect()
}
