use std::collections::HashSet;

use crate::vector3::{Vector3, Vector3f};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    stones: Vec<Stone>,
    area: (Vector3f, Vector3f),
}

#[derive(Debug)]
struct Stone {
    position: Vector3,
    velocity: Vector3,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 2,
            answer_b: 47,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            stones: raw.iter().map(|line| line.parse().unwrap()).collect(),
            area: if raw.len() < 10 {
                (Vector3f::new(7.0, 7.0, 0.0), Vector3f::new(27.0, 27.0, 1.0))
            } else {
                (
                    Vector3f::new(200000000000000.0, 200000000000000.0, 0.0),
                    Vector3f::new(400000000000000.0, 400000000000000.0, 0.0),
                )
            },
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        let stones: Vec<_> = self.stones.iter().map(|s| s.xy()).collect();
        for (i, &a) in stones.iter().enumerate() {
            for &b in stones.iter().skip(i + 1) {
                let Some((scale_a, scale_b)) = Vector3f::closest_points(a, b) else {
                    continue;
                };
                if scale_a < 0.0 || scale_b < 0.0 {
                    continue;
                }
                let intersection = a.0 + a.1 * scale_a;
                if intersection.contained_in(self.area.0, self.area.1) {
                    count += 1;
                }
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl Stone {
    const fn xy(&self) -> (Vector3f, Vector3f) {
        (
            Vector3f::new(self.position.x as f64, self.position.y as f64, 0.0),
            Vector3f::new(self.velocity.x as f64, self.velocity.y as f64, 0.0),
        )
    }
}

impl std::str::FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" @ ").unwrap();
        let left: Vec<_> = left.split(", ").map(|s| s.trim()).collect();
        let right: Vec<_> = right.split(", ").map(|s| s.trim()).collect();
        Ok(Stone {
            position: Vector3::new(
                left[0].parse().unwrap(),
                left[1].parse().unwrap(),
                left[2].parse().unwrap(),
            ),
            velocity: Vector3::new(
                right[0].parse().unwrap(),
                right[1].parse().unwrap(),
                right[2].parse().unwrap(),
            ),
        })
    }
}

crate::solution::test_solution!();
