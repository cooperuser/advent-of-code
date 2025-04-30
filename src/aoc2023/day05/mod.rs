use std::{ops::Range, str::FromStr};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    seeds: Vec<i64>,
    maps: Vec<Vec<Mapping>>,
}

struct Mapping {
    range: Range<i64>,
    offset: i64,
}

trait Convertable<T> {
    fn convert(&self, value: T) -> Vec<T>;
}

impl Convertable<i64> for Vec<Mapping> {
    fn convert(&self, value: i64) -> Vec<i64> {
        for mapping in self {
            if mapping.range.contains(&value) {
                return vec![value + mapping.offset];
            }
        }
        vec![value]
    }
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 35,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let (seeds, map_sections) = raw.split_once(|line| line.is_empty()).unwrap();
        let map_sections: Vec<_> = map_sections.split(|line| line.is_empty()).collect();
        let seeds = seeds[0].split_once(": ").unwrap().1;
        let seeds = seeds
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut maps = Vec::new();
        for map_section in map_sections {
            let mut map = Vec::new();
            for line in map_section.iter().skip(1) {
                map.push(line.parse().unwrap());
            }
            maps.push(map);
        }

        Self {
            raw: raw.clone(),
            seeds,
            maps,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut spots = self.seeds.clone();
        for map in &self.maps {
            spots = spots.iter().flat_map(|&seed| map.convert(seed)).collect();
        }
        Some(*spots.iter().min().unwrap())
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let parts: Vec<i64> = parts.iter().map(|p| p.parse().unwrap()).collect();
        Ok(Mapping {
            range: parts[1]..parts[1] + parts[2],
            offset: parts[0] - parts[1],
        })
    }
}

crate::solution::test_solution!();
