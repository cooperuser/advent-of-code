use crate::{solution::Run, Solution};

mod day22;

pub const DAYS: &[fn(bool) -> Run] = &[day22::Day::run];
