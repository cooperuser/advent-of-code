use crate::{solution::Run, Solution};

mod day21;
mod day22;

pub const DAYS: &[fn(bool) -> Run] = &[day21::Day::run, day22::Day::run];
