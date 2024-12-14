use crate::{solution::Run, Solution};

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

pub const DAYS: &[fn(bool) -> Run] = &[
    day01::Day::run,
    day02::Day::run,
    day03::Day::run,
    day04::Day::run,
    day05::Day::run,
    day06::Day::run,
    day07::Day::run,
    day08::Day::run,
    day09::Day::run,
    day10::Day::run,
    day11::Day::run,
    day12::Day::run,
    day13::Day::run,
    day14::Day::run,
    day15::Day::run,
];
