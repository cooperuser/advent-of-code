use std::{collections::HashSet, str::FromStr};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    seeds: HashSet<usize>,
    maps: Vec<Vec<Range>>,
}

#[derive(Debug)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn contains(&self, value: usize) -> bool {
        value >= self.source && value < self.source + self.length
    }

    fn offset(&self, value: usize) -> usize {
        self.destination + value - self.source
    }
}

impl crate::solution::Solution<usize, i64> for Day {
    fn meta() -> crate::solution::Meta<usize, i64> {
        crate::solution::Meta::<usize, i64> {
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
        let seeds: HashSet<usize> = seeds
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

    fn part_a(&self) -> Option<usize> {
        let mut spots = self.seeds.clone();
        for map in self.maps.iter() {
            let mut next = HashSet::new();
            'spot: for spot in spots {
                for range in map {
                    if range.contains(spot) {
                        next.insert(range.offset(spot));
                        continue 'spot;
                    }
                }
                next.insert(spot);
            }
            spots = next;
        }
        Some(*spots.iter().min().unwrap())
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<usize> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            source: s[1],
            destination: s[0],
            length: s[2],
        })
    }
}

crate::solution::test_solution!();
