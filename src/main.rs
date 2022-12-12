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

fn main() {
    use day12::*;

    let sample = Solution::new(split(SAMPLE));
    let real = Solution::new(split(INPUT));

    let part_a = sample.part_a();
    if part_a == SAMPLE_A {
        println!("part_a:\t{}", real.part_a());
    } else {
        println!("part_a: failed!");
        println!("\texpected: {}", SAMPLE_A);
        println!("\treceived: {}", part_a);
    }

    let part_b = sample.part_b();
    if part_b == SAMPLE_B {
        println!("part_b:\t{}", real.part_b());
    } else {
        println!("part_b: failed!");
        println!("\texpected: {}", SAMPLE_B);
        println!("\treceived: {}", part_b);
    }
}

pub fn split(input: &str) -> Vec<String> {
    input.trim_end().split("\n").map(|s| s.to_string()).collect()
}
