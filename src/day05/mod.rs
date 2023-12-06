#![allow(dead_code)]

use std::{collections::HashMap, ops::Range};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 35;
pub const SAMPLE_B: i64 = 46;

trait Helper {
    fn convert(&self, maps: Option<&Vec<Mapping>>) -> Vec<Range<i64>>;
    fn offset(&self, delta: i64) -> Range<i64>;
}

impl Helper for std::ops::Range<i64> {
    fn convert(&self, maps: Option<&Vec<Mapping>>) -> Vec<std::ops::Range<i64>> {
        #![allow(clippy::collapsible_else_if)]
        let maps = maps.unwrap();
        let mut range = self.clone();
        let mut ranges = Vec::new();
        for map in maps {
            let a = range.start;
            let b = range.end;
            let c = map.range.start;
            let d = map.range.end;
            if b < c { break; }
            if a < d {
                if a < c {
                    if b < d {
                        // a---b
                        //   c---d
                        ranges.push(a..c);
                        ranges.push((c..b).offset(map.delta));
                        return ranges;
                    } else {
                        // a-------b
                        //   c---d
                        ranges.push(a..c);
                        ranges.push((c..d).offset(map.delta));
                        range.start = d;
                    }
                } else {
                    if b < d {
                        //   a---b
                        // c-------d
                        ranges.push((a..b).offset(map.delta));
                        return ranges;
                    } else {
                        //   a---b
                        // c---d
                        ranges.push((a..d).offset(map.delta));
                        range.start = d;
                    }
                }
            }
        }
        if range.start < range.end {
            ranges.push(range);
        }
        ranges
    }

    fn offset(&self, delta: i64) -> Range<i64> {
        Range { start: self.start + delta, end: self.end + delta }
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    range: Range<i64>,
    delta: i64,
}

fn convert(num: i64, maps: Option<&Vec<Mapping>>) -> i64 {
    let maps = maps.unwrap();
    for map in maps {
        if num >= map.range.start && num < map.range.end {
            return num + map.delta;
        }
    }
    num
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    seeds: Vec<i64>,
    maps: HashMap<String, Vec<Mapping>>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut seeds = Vec::new();
        let mut maps = HashMap::new();
        for group in raw.split(|line| line.is_empty()) {
            if group.is_empty() { continue; }
            else if group.len() == 1 {
                seeds = group
                    .first().unwrap()
                    .split_once(": ").unwrap().1
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
            } else {
                let map: Vec<&str> = group.first().unwrap().split_once(' ').unwrap().0.split('-').collect();
                let (from, _) = (map[0].to_string(), map[2].to_string());
                let mut mapping = Vec::new();
                for line in &group[1..] {
                    let nums: Vec<i64> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
                    mapping.push(Mapping {
                        range: nums[1]..nums[1] + nums[2],
                        delta: nums[0] - nums[1],
                    })
                }
                mapping.sort_by_key(|m| m.range.start);
                maps.insert(from, mapping);
            }
        }
        Self {
            raw: raw.clone(),
            seeds,
            maps,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let seeds = &self.seeds;
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("seed"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("soil"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("fertilizer"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("water"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("light"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("temperature"))).collect();
        let seeds: Vec<i64> = seeds.iter().map(|seed| convert(*seed, self.maps.get("humidity"))).collect();
        Some(*seeds.iter().min().unwrap())
    }

    pub fn part_b(&self) -> Option<i64> {
        let ranges: Vec<Range<i64>> = self.seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("seed"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("soil"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("fertilizer"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("water"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("light"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("temperature"))).collect();
        let ranges: Vec<Range<i64>> = ranges.iter().flat_map(|range| range.convert(self.maps.get("humidity"))).collect();
        Some(ranges.iter().map(|r| r.start).min().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a().unwrap_or(0), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b().unwrap_or(0), SAMPLE_B);
    }
}
