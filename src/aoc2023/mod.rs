use crate::{solution::Run, Solution};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub const DAYS: &[fn(bool) -> Run] = &[
    day01::Day::run,
    day02::Day::run,
    day03::Day::run,
    day04::Day::run,
    day05::Day::run,
    day06::Day::run,
    day07::Day::run,
    day08::Day::run,
    day21::Day::run,
    day22::Day::run,
    day23::Day::run,
    day24::Day::run,
    day25::Day::run,
];
