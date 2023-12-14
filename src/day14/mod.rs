use std::collections::{HashSet, HashMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 136;
pub const ANSWER_B: i64 = 64;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: Map,
}

#[derive(Default, Clone)]
struct Map {
    cubes: HashSet<(i64, i64)>,
    rocks: HashSet<(i64, i64)>,
    size: (i64, i64),
}

impl Map {
    fn debug(&self) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let pos = (row, col);
                let cube = self.cubes.contains(&pos);
                let rock = self.rocks.contains(&pos);
                if cube && rock {
                    panic!("error");
                } else if cube {
                    print!("#");
                } else if rock {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn tilt_north(&mut self) {
        for _ in 0..self.size.0 {
            let mut rocks = HashSet::new();
            for row in 0..self.size.0 {
                for col in 0..self.size.1 {
                    let pos = (row, col);
                    let next = (row - 1, col);
                    if self.rocks.contains(&pos) {
                        if self.cubes.contains(&next) || self.rocks.contains(&next) || row < 1 {
                            rocks.insert(pos);
                            continue;
                        }
                        rocks.insert(next);
                    }
                }
            }
            self.rocks = rocks;
        }
    }

    fn tilt_south(&mut self) {
        for _ in 0..self.size.0 {
            let mut rocks = HashSet::new();
            for row in (0..self.size.0).rev() {
                for col in 0..self.size.1 {
                    let pos = (row, col);
                    let next = (row + 1, col);
                    if self.rocks.contains(&pos) {
                        if self.cubes.contains(&next) || self.rocks.contains(&next) || row == self.size.0 - 1 {
                            rocks.insert(pos);
                            continue;
                        }
                        rocks.insert(next);
                    }
                }
            }
            self.rocks = rocks;
        }
    }

    fn tilt_west(&mut self) {
        for _ in 0..self.size.1 {
            let mut rocks = HashSet::new();
            for col in 0..self.size.1 {
                for row in 0..self.size.0 {
                    let pos = (row, col);
                    let next = (row, col - 1);
                    if self.rocks.contains(&pos) {
                        if self.cubes.contains(&next) || self.rocks.contains(&next) || col < 1 {
                            rocks.insert(pos);
                            continue;
                        }
                        rocks.insert(next);
                    }
                }
            }
            self.rocks = rocks;
        }
    }

    fn tilt_east(&mut self) {
        for _ in 0..self.size.1 {
            let mut rocks = HashSet::new();
            for col in (0..self.size.1).rev() {
                for row in 0..self.size.0 {
                    let pos = (row, col);
                    let next = (row, col + 1);
                    if self.rocks.contains(&pos) {
                        if self.cubes.contains(&next) || self.rocks.contains(&next) || col == self.size.1 - 1 {
                            rocks.insert(pos);
                            continue;
                        }
                        rocks.insert(next);
                    }
                }
            }
            self.rocks = rocks;
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn serialize(&self) -> String {
        let mut values = Vec::new();
        for row in 0..self.size.0 {
            let mut value = 0i128;
            for col in 0..self.size.1 {
                if self.rocks.contains(&(row, col)) {
                    value |= 1 << col;
                }
            }
            values.push(value.to_string());
        }
        values.join("_")
    }

    fn load_north(&self) -> i64 {
        let mut load = 0;
        for rock in &self.rocks {
            load += self.size.0 - rock.0;
        }
        load
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut cubes = HashSet::new();
        let mut rocks = HashSet::new();
        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row as i64, col as i64);
                match ch {
                    '#' => {
                        cubes.insert(pos);
                    },
                    'O' => {
                        rocks.insert(pos);
                    }
                    _ => ()
                }
            }
        }
        Self {
            raw: raw.clone(),
            map: Map {
                rocks,
                cubes,
                size: (raw.len() as i64, raw[0].len() as i64),
            }
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut map = self.map.clone();
        map.tilt_north();
        Some(map.load_north())
    }

    pub fn part_b(&self) -> Option<i64> {
        let count = 1_000_000_000;
        let mut map = self.map.clone();
        let mut seen = HashMap::new();
        let mut i = 0i128;
        while i < count {
            let serial = map.serialize();
            if let Some(previous) = seen.get(&serial) {
                let loop_length = i - previous;
                i += loop_length * (count / loop_length);
                while i > count {
                    i -= loop_length;
                }
                break;
            }
            seen.insert(serial, i);
            map.cycle();
            i += 1;
        }
        while i < count {
            map.cycle();
            i += 1;
        }
        Some(map.load_north())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
