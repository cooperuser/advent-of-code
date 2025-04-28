use crate::{solution::Run, Solution};

mod day01;
mod day02;
mod day03;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub const DAYS: &[fn(bool) -> Run] = &[
    day01::Day::run,
    day02::Day::run,
    day03::Day::run,
    day21::Day::run,
    day22::Day::run,
    day23::Day::run,
    day24::Day::run,
    day25::Day::run,
];
