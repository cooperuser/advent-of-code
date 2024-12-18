pub type Run = Option<(
    std::time::Duration,
    std::time::Duration,
    std::time::Duration,
)>;

pub trait Solution<T: std::fmt::Display + Eq, U: std::fmt::Display + Eq> {
    fn meta() -> Meta<T, U>
    where
        Self: Sized;
    fn new(raw: Vec<String>) -> Self
    where
        Self: Sized;
    fn part_a(&self) -> Option<T>;
    fn part_b(&self) -> Option<U>;
    fn run(silenced: bool) -> Run
    where
        Self: Sized,
    {
        let meta = Self::meta();
        let start = std::time::Instant::now();
        let real = Self::new(crate::split(meta.input));
        let duration = start.elapsed();

        if silenced {
            let start = std::time::Instant::now();
            real.part_a();
            let a = start.elapsed();
            let start = std::time::Instant::now();
            real.part_b();
            let b = start.elapsed();
            return Some((duration, a, b));
        }

        let sample_a = Self::new(crate::split(meta.sample_a));
        let sample_b = Self::new(crate::split(meta.sample_b));
        println!("parse :\t{:?}\n", duration);
        match sample_a.part_a() {
            Some(received) if received == meta.answer_a => {
                let start = std::time::Instant::now();
                let answer = real.part_a();
                let duration = start.elapsed();
                println!("part_a:\t{}", answer.unwrap());
                println!(" ** in:\t{:?}", duration);
            }
            Some(received) => {
                println!("part_a: failed!");
                println!("\texpected: {}", meta.answer_a);
                println!("\treceived: {}", received);
            }
            None => {
                println!("part_a: unsolved!");
            }
        }

        match sample_b.part_b() {
            Some(received) if received == meta.answer_b => {
                let start = std::time::Instant::now();
                let answer = real.part_b();
                let duration = start.elapsed();
                println!("part_b:\t{}", answer.unwrap());
                println!(" ** in:\t{:?}", duration);
            }
            Some(received) => {
                println!("part_b: failed!");
                println!("\texpected: {}", meta.answer_b);
                println!("\treceived: {}", received);
            }
            None => {
                println!("part_b: unsolved!");
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct Meta<T, U> {
    pub input: String,
    pub sample_a: String,
    pub sample_b: String,
    pub answer_a: T,
    pub answer_b: U,
}

macro_rules! test_solution {
    () => {
        #[cfg(test)]
        mod solution {
            use super::*;
            use crate::solution::Solution;

            #[test]
            fn part_a() {
                let meta = Day::meta();
                let solution = Day::new(crate::split(meta.sample_a));
                assert_eq!(solution.part_a(), Some(meta.answer_a));
            }

            #[test]
            fn part_b() {
                let meta = Day::meta();
                let solution = Day::new(crate::split(meta.sample_b));
                assert_eq!(solution.part_b(), Some(meta.answer_b));
            }
        }
    };
}

pub(crate) use test_solution;
