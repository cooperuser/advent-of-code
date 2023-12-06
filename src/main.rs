mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    use day05::*;

    let sample = Solution::new(split(SAMPLE));
    let start = std::time::Instant::now();
    let real = Solution::new(split(INPUT));
    let duration = start.elapsed();
    println!("parse :\t{:?}\n", duration);

    match sample.part_a() {
        Some(SAMPLE_A) => {
            let start = std::time::Instant::now();
            let answer = real.part_a();
            let duration = start.elapsed();
            println!("part_a:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        },
        Some(received) => {
            println!("part_a: failed!");
            println!("\texpected: {}", SAMPLE_A);
            println!("\treceived: {}", received);
        },
        None => {
            println!("part_a: unsolved!");
        }
    }

    match sample.part_b() {
        Some(SAMPLE_B) => {
            let start = std::time::Instant::now();
            let answer = real.part_b();
            let duration = start.elapsed();
            println!("part_b:\t{}", answer.unwrap());
            println!(" ** in:\t{:?}", duration);
        },
        Some(received) => {
            println!("part_b: failed!");
            println!("\texpected: {}", SAMPLE_B);
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
