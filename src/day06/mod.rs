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

impl Race {
    fn count(&self) -> i64 {
        let a = -1f64;
        let b = self.time as f64;
        let c = -self.distance as f64;

        let mut high = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor() as i64;
        let mut low = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil() as i64;
        if high * (self.time - high) == self.distance { high -= 1; }
        if low * (self.time - low) == self.distance { low += 1; }

        high - low + 1
    }
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
        Some(self.races.iter().map(|r| r.count()).product())
    }

    pub fn part_b(&self) -> Option<i64> {
        Some(self.race.count())
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
