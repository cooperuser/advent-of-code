#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 288;
pub const SAMPLE_B: i64 = 71503;

#[derive(Debug, Default)]
struct Race {
    time: i64,
    distance: i64,
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    races: Vec<Race>,
    race: Race
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut races = Vec::new();
        let times: Vec<_> = raw[0].split_whitespace().collect();
        let dists: Vec<_> = raw[1].split_whitespace().collect();
        for (i, (time, distance)) in times.iter().zip(&dists).enumerate() {
            if i == 0 { continue; }
            races.push(Race {
                time: time.parse::<i64>().unwrap(),
                distance: distance.parse::<i64>().unwrap(),
            });
        }
        Self {
            raw: raw.clone(),
            races,
            race: Race {
                time: times[1..].join("").parse().unwrap(),
                distance: dists[1..].join("").parse().unwrap(),
            }
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut product = 1;
        for race in &self.races {
            product *= (1..race.time)
                .filter(|i| i * (race.time - i) > race.distance)
                .count() as i64;
        }
        Some(product)
    }

    pub fn part_b(&self) -> Option<i64> {
        let count = (1..self.race.time)
            .filter(|i| i * (self.race.time - i) > self.race.distance)
            .count() as i64;
        Some(count)
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
