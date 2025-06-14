use crate::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    races: Vec<Race>,
    race: Race,
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn count(&self) -> usize {
        let a = -1f64;
        let b = self.time as f64;
        let c = -self.distance as f64;

        let mut high = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor() as i64;
        let mut low = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil() as i64;
        if high * (self.time - high) == self.distance {
            high -= 1;
        }
        if low * (self.time - low) == self.distance {
            low += 1;
        }

        (high - low + 1) as usize
    }
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 288,
            answer_b: 71503,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let times: Vec<i64> = raw[0]
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();
        let distances: Vec<i64> = raw[1]
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            raw: raw.clone(),
            races: times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
            race: Race {
                time: raw[0]
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("")
                    .parse()
                    .unwrap(),
                distance: raw[1]
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("")
                    .parse()
                    .unwrap(),
            },
        }
    }

    fn part_a(&self) -> Option<usize> {
        Some(self.races.iter().map(|r| r.count()).product())
    }

    fn part_b(&self) -> Option<usize> {
        Some(self.race.count())
    }
}

crate::solution::test_solution!();
