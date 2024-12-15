use crate::{solution::Run, Solution};

mod day21;
mod day22;
mod day23;
mod day24;

pub const DAYS: &[fn(bool) -> Run] = &[
    day21::Day::run,
    day22::Day::run,
    day23::Day::run,
    day24::Day::run,
];
