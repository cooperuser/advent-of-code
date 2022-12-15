#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 26;
pub const SAMPLE_B: i64 = 56000011;

type Position = (i64, i64);

fn get_dist(a: Position, b: Position) -> i64 {
    let diff = (b.0 - a.0, b.1 - a.1);
    diff.0.abs() + diff.1.abs()
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    sensors: Vec<Position>,
    beacons: Vec<Position>,
    closest: HashMap<Position, Position>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut sensors = vec![];
        let mut beacons = vec![];
        let mut closest = HashMap::new();
        for line in &raw {
            if line.is_empty() { continue }
            let words: Vec<&str> = line.split_whitespace().collect();
            let sensor = (words[2], words[3]);
            let beacon = (words[8], words[9]);
            let sensor = (
                sensor.0.split_once('=').unwrap().1.split_once(',').unwrap().0.parse::<i64>().unwrap(),
                sensor.1.split_once('=').unwrap().1.split_once(':').unwrap().0.parse::<i64>().unwrap()
            );
            let beacon = (
                beacon.0.split_once('=').unwrap().1.split_once(',').unwrap().0.parse::<i64>().unwrap(),
                beacon.1.split_once('=').unwrap().1.parse::<i64>().unwrap()
            );
            sensors.push(sensor);
            beacons.push(beacon);
            closest.insert(sensor, beacon);
            
        }
        Self {
            raw: raw.clone(),
            sensors,
            beacons,
            closest,
            ..Default::default()
        }
    }

    fn get_closest(&self, pos: Position, list: &Vec<Position>) -> Position {
        let mut min_dist = i64::MAX;
        let mut min_obj = (0, 0);
        for obj in list {
            let diff = (obj.0 - pos.0, obj.1 - pos.1);
            let dist = diff.0.abs() + diff.1.abs();
            if dist < min_dist {
                min_dist = dist;
                min_obj = *obj;
            }
        }
        min_obj
    }

    pub fn part_a(&self) -> i32 {
        let target_row = if self.sensors.len() < 20 { 10 } else { 2000000 };
        let mut spots: HashSet<i64> = HashSet::new();
        for sensor in &self.sensors {
            let beacon = self.closest.get(sensor).unwrap();
            let radius = get_dist(*sensor, *beacon);
            let dist = (sensor.1 - target_row).abs();
            if dist > radius { continue }

            let remainder = radius - dist;
            let start = sensor.0 - remainder;
            let end = sensor.0 + remainder;

            for x in start..=end {
                if (x, target_row) == *beacon { continue }
                spots.insert(x);
            }
        }
        spots.len() as i32
    }

    pub fn part_b(&self) -> i64 {
        let max_pos = if self.sensors.len() < 20 { 20 } else { 4000000 } as i64;
        let mut data = vec![vec![0..=max_pos]; max_pos as usize + 1];
        for sensor in &self.sensors {
            let beacon = self.closest.get(sensor).unwrap();
            let radius = get_dist(*sensor, *beacon);
            let top = 0.max(sensor.1 - radius);
            let bottom = max_pos.min(sensor.1 + radius);

            for row in top..=bottom {
                let dist = (sensor.1 - row).abs();
                let min_x = 0.max(sensor.0 - (radius - dist));
                let max_x = max_pos.min(sensor.0 + (radius - dist));

                let mut range = Vec::new();
                for r in &data[row as usize] {
                    let start = *r.start();
                    if start > max_x {
                        range.push(r.clone());
                        continue;
                    }

                    let end = *r.end();
                    if end < min_x {
                        range.push(r.clone());
                        continue;
                    }

                    if start < min_x {
                        range.push(start..=min_x - 1);
                    }
                    if end > max_x {
                        range.push(max_x + 1..=end);
                    }
                }

                data[row as usize] = range;
            }
        }

        for (y, range) in data.iter().enumerate() {
            if !range.is_empty() {
                let x = *range[0].start();
                return x * 4000000 + y as i64;
            }
        }

        0
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
