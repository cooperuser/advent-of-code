#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 62;
pub const ANSWER_B: i64 = 952408144115;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    plans: Vec<Plan>,
}

type Pos = (i64, i64);

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn mul(&self, value: i64) -> Pos {
        match self {
            Dir::North => (-value, 0),
            Dir::South => (value, 0),
            Dir::East => (0, value),
            Dir::West => (0, -value),
        }
    }
}

#[derive(Debug)]
struct Plan {
    direction: Dir,
    distance: i64,
    color: String,
}

fn get_area(vertices: &[Pos]) -> i64 {
    // Shoelace formula
    let area: i64 = vertices
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[1].0 * w[0].1)
        .sum();
    // Roughly the number of edge tiles
    let perimeter: i64 = vertices
        .windows(2)
        .map(|w| (w[1].0 - w[0].0 + w[1].1 - w[0].1).abs())
        .sum();

    (area.abs() + perimeter) / 2 + 1
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut plans = Vec::new();
        for line in &raw {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                continue;
            }
            plans.push(Plan {
                direction: match parts[0] {
                    "U" => Dir::North,
                    "D" => Dir::South,
                    "R" => Dir::East,
                    "L" => Dir::West,
                    _ => panic!("Unknown direction {}", parts[0]),
                },
                distance: parts[1].parse().unwrap(),
                color: parts[2][1..parts[2].len() - 1].to_string()
            })
        }
        Self {
            raw: raw.clone(),
            plans,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut pos = (0, 0);
        let mut vertices = Vec::from([pos]);
        for plan in &self.plans {
            let offset = plan.direction.mul(plan.distance);
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            vertices.push(pos);
        }

        Some(get_area(&vertices))
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut pos = (0, 0);
        let mut vertices = Vec::from([pos]);
        for plan in &self.plans {
            let distance: i64 = plan.color[1..6]
                .chars()
                .enumerate()
                .map(|(i, c)| (c.to_digit(16).unwrap() << (4 * (4 - i))) as i64)
                .sum();
            let direction = match &plan.color[6..=6] {
                "0" => Dir::East,
                "1" => Dir::South,
                "2" => Dir::West,
                "3" => Dir::North,
                _ => panic!("Unknown direction"),
            };

            let offset = direction.mul(distance);
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            vertices.push(pos);
        }

        Some(get_area(&vertices))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
