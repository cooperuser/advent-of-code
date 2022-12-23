#![allow(dead_code)]

use std::collections::{HashSet, HashMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 110;
pub const SAMPLE_B: i32 = 20;

type Pos = (i32, i32);

const N: Pos = (-1, 0);
const S: Pos = (1, 0);
const E: Pos = (0, 1);
const W: Pos = (0, -1);
const NE: Pos = (N.0, E.1);
const SE: Pos = (S.0, E.1);
const NW: Pos = (N.0, W.1);
const SW: Pos = (S.0, W.1);
const CARDINAL: [Pos; 4] = [N, S, W, E];
const DIRECTIONS: [Pos; 8] = [N, S, W, E, NE, SE, NW, SW];

#[derive(Debug)]
struct Adjacents {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    northeast: bool,
    southeast: bool,
    northwest: bool,
    southwest: bool,
}

impl From<[bool; 8]> for Adjacents {
    fn from(list: [bool; 8]) -> Self {
        Self {
            north: list[0],
            south: list[1],
            west: list[2],
            east: list[3],
            northeast: list[4],
            southeast: list[5],
            northwest: list[6],
            southwest: list[7],
        }
    }
}

impl Adjacents {
    fn has_north(&self) -> bool {
        self.north || self.northeast || self.northwest
    }

    fn has_south(&self) -> bool {
        self.south || self.southeast || self.southwest
    }

    fn has_east(&self) -> bool {
        self.east || self.northeast || self.southeast
    }

    fn has_west(&self) -> bool {
        self.west || self.northwest || self.southwest
    }

    fn has_any(&self) -> bool {
        self.north
        || self.south
        || self.east
        || self.west
        || self.northeast
        || self.northwest
        || self.southeast
        || self.southwest
    }

    fn has(&self, index: usize) -> bool {
        match index {
            0 => self.has_north(),
            1 => self.has_south(),
            2 => self.has_west(),
            3 => self.has_east(),
            _ => panic!()
        }
    }
}

fn get_min_max(map: &HashSet<Pos>) -> (Pos, Pos) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for (y, x) in map {
        min = (min.0.min(*y), min.1.min(*x));
        max = (max.0.max(*y), max.1.max(*x));
    }
    (min, max)
}

fn get_blank_spaces(map: &HashSet<Pos>) -> i32 {
    let (min, max) = get_min_max(map);
    let mut count = 0;

    for y in min.0..=max.0 {
        for x in min.1..=max.1 {
            if !map.contains(&(y, x)) {
                count += 1
            }
        }
    }

    count
}

fn _debug_map(map: &HashSet<Pos>) {
    let (min, max) = get_min_max(map);

    for y in min.0..=max.0 {
        for x in min.1..=max.1 {
            if map.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: HashSet<Pos>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashSet::new();
        for (y, line) in raw.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '#' { continue; }
                let (y, x) = (y as i32, x as i32);
                map.insert((y, x));
            }
        }
        Self {
            raw: raw.clone(),
            map,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut map = self.map.clone();
        for round in 0..10 {
            let mut new_map: HashMap<Pos, Vec<Pos>> = HashMap::new();
            for elf in &map {
                let adjacents = Adjacents::from(DIRECTIONS.map(|dir| {
                    let pos = (elf.0 + dir.0, elf.1 + dir.1);
                    map.contains(&pos)
                }));

                if !adjacents.has_any() {
                    new_map.insert(*elf, vec![*elf]);
                } else {
                    let mut added = false;
                    for index in 0..4 {
                        let i = (round + index) % 4;
                        let dir = CARDINAL[i];
                        let has = adjacents.has(i);
                        if !has {
                            let pos = (elf.0 + dir.0, elf.1 + dir.1);
                            if new_map.contains_key(&pos) {
                                new_map.get_mut(&pos).unwrap().push(*elf);
                            } else {
                                new_map.insert(pos, vec![*elf]);
                            }
                            added = true;
                            break;
                        }
                    }
                    if !added {
                        if new_map.contains_key(&elf) {
                            new_map.get_mut(&elf).unwrap().push(*elf);
                        } else {
                            new_map.insert(*elf, vec![*elf]);
                        }
                    }
                }
            }

            map.clear();
            for (pos, elves) in &new_map {
                if elves.len() == 1 {
                    map.insert(*pos);
                } else {
                    for elf in elves {
                        map.insert(*elf);
                    }
                }
            }
        }
        get_blank_spaces(&map)
    }

    pub fn part_b(&self) -> i32 {
        let mut map = self.map.clone();
        let mut round = 0;
        loop {
            let mut new_map: HashMap<Pos, Vec<Pos>> = HashMap::new();
            let mut moved = false;
            for elf in &map {
                let adjacents = Adjacents::from(DIRECTIONS.map(|dir| {
                    let pos = (elf.0 + dir.0, elf.1 + dir.1);
                    map.contains(&pos)
                }));

                if !adjacents.has_any() {
                    new_map.insert(*elf, vec![*elf]);
                } else {
                    let mut added = false;
                    for index in 0..4 {
                        let i = (round + index) % 4;
                        let dir = CARDINAL[i];
                        let has = adjacents.has(i);
                        if !has {
                            let pos = (elf.0 + dir.0, elf.1 + dir.1);
                            if new_map.contains_key(&pos) {
                                new_map.get_mut(&pos).unwrap().push(*elf);
                            } else {
                                new_map.insert(pos, vec![*elf]);
                                moved = true;
                            }
                            added = true;
                            break;
                        }
                    }
                    if !added {
                        if new_map.contains_key(&elf) {
                            new_map.get_mut(&elf).unwrap().push(*elf);
                        } else {
                            new_map.insert(*elf, vec![*elf]);
                        }
                    }
                }
            }

            map.clear();
            for (pos, elves) in &new_map {
                if elves.len() == 1 {
                    map.insert(*pos);
                } else {
                    for elf in elves {
                        map.insert(*elf);
                    }
                }
            }
            round += 1;
            if !moved { break; }
        }
        round as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
