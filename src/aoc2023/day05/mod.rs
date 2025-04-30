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
    fn convert(&self, value: &T) -> Vec<T>;
}

impl Convertable<i64> for Vec<Mapping> {
    fn convert(&self, value: &i64) -> Vec<i64> {
        for mapping in self {
            if mapping.range.contains(value) {
                return vec![value + mapping.offset];
            }
        }
        vec![*value]
    }
}

impl Convertable<Range<i64>> for Vec<Mapping> {
    fn convert(&self, value: &Range<i64>) -> Vec<Range<i64>> {
        #![allow(clippy::collapsible_else_if)]
        let mut range = value.clone();
        let mut ranges = Vec::new();

        for map in self {
            let a = range.start;
            let b = range.end;
            let c = map.range.start;
            let d = map.range.end;

            if b < c {
                // a---b
                //       c---d
                break;
            }

            if a < d {
                if a < c {
                    if b < d {
                        // a---b
                        //   c---d
                        ranges.push(a..c);
                        ranges.push(c + map.offset..b + map.offset);
                        return ranges;
                    } else {
                        // a-------b
                        //   c---d
                        ranges.push(a..c);
                        ranges.push(c + map.offset..d + map.offset);
                        range.start = d;
                    }
                } else {
                    if b < d {
                        //   a---b
                        // c-------d
                        ranges.push(a + map.offset..b + map.offset);
                        return ranges;
                    } else {
                        //   a---b
                        // c---d
                        ranges.push(a + map.offset..d + map.offset);
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
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 35,
            answer_b: 46,
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
            let mut map: Vec<Mapping> = Vec::new();
            for line in map_section.iter().skip(1) {
                map.push(line.parse().unwrap());
            }
            map.sort_by_key(|m| m.range.start);
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
            spots = spots.iter().flat_map(|seed| map.convert(seed)).collect();
        }
        Some(*spots.iter().min().unwrap())
    }

    fn part_b(&self) -> Option<i64> {
        let mut spots: Vec<_> = self.seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();
        for map in &self.maps {
            spots = spots.iter().flat_map(|range| map.convert(range)).collect();
        }
        Some(spots.iter().map(|r| r.start).min().unwrap())
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
