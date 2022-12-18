#![allow(dead_code)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 64;
pub const SAMPLE_B: i32 = 58;

type Position = (i32, i32, i32);
const DIRS: [Position; 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    cubes: Vec<(i32, i32, i32)>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut cubes = vec![];
        for line in &raw {
            if line.is_empty() { continue }
            let nums = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let cube = (nums[0], nums[1], nums[2]);
            cubes.push(cube);
        }
        Self {
            raw: raw.clone(),
            cubes,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut shape: HashSet<Position> = HashSet::new();
        let mut sides = 0;
        for cube in &self.cubes {
            shape.insert(*cube);
            sides += 6;
            for dir in DIRS {
                let spot = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);
                if shape.contains(&spot) { sides -= 2; }
            }
        }
        sides
    }

    pub fn part_b(&self) -> i32 {
        let mut shape: HashSet<Position> = HashSet::new();
        let mut min = (i32::MAX, i32::MAX, i32::MAX);
        let mut max = (i32::MIN, i32::MIN, i32::MIN);
        for cube in &self.cubes {
            shape.insert(*cube);
            min.0 = min.0.min(cube.0);
            min.1 = min.1.min(cube.1);
            min.2 = min.2.min(cube.2);
            max.0 = max.0.max(cube.0);
            max.1 = max.1.max(cube.1);
            max.2 = max.2.max(cube.2);
        }

        let mut shell = HashSet::new();
        let mut stack = vec![(min.0 - 1, min.1 - 1, min.2 - 1)];
        let mut checked = HashSet::new();
        while !stack.is_empty() {
            let cube = stack.remove(0);
            shell.insert(cube);
            for dir in DIRS {
                let spot = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);
                if spot.0 < min.0 - 1 { continue }
                if spot.0 > max.0 + 1 { continue }
                if spot.1 < min.1 - 1 { continue }
                if spot.1 > max.1 + 1 { continue }
                if spot.2 < min.2 - 1 { continue }
                if spot.2 > max.2 + 1 { continue }
                if checked.contains(&spot) { continue }
                checked.insert(spot);
                if shape.contains(&spot) { continue }
                stack.push(spot);
            }
        }

        let mut sides = 0;
        let mut building = HashSet::new();
        for cube in &shell {
            building.insert(*cube);
            sides += 6;
            for dir in DIRS {
                let spot = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);
                if building.contains(&spot) { sides -= 2; }
            }
        }

        let diff = (max.0 - min.0 + 3, max.1 - min.1 + 3, max.2 - min.2 + 3);
        let exterior = 2 * (diff.0 * diff.1 + diff.1 * diff.2 + diff.2 * diff.0);
        sides - exterior
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
