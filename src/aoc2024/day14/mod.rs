use std::rc::Rc;

use crate::vector::{Vector, VectorMap, VectorSet};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    robots: Vec<Robot>,
    size: Vector,
}

#[derive(Clone)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 12,
            answer_b: 1,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self {
            raw: raw.clone(),
            robots: raw.iter().map(|line| line.parse().unwrap()).collect(),
            size: match raw.len() {
                12 => Vector::new(11, 7),
                _ => Vector::new(101, 103),
            },
        }
    }

    fn part_a(&self) -> Option<i64> {
        let half = self.size / 2;
        let plus_one = half + Vector::new(1, 1);
        let mut quadrants: VectorMap<i64> = VectorMap::filled_with_value(Vector::new(2, 2), 0);

        for robot in self.robots.iter() {
            let pos = robot.position + robot.velocity * 100;
            let pos = pos.rem_euclid(self.size);
            if pos.x != half.x && pos.y != half.y {
                let quadrant = Vector::new(pos.x / plus_one.x, pos.y / plus_one.y);
                *quadrants.get_mut(quadrant).unwrap() += 1;
            }
        }

        Some(quadrants.iter().map(|(_, v)| v).product())
    }

    fn part_b(&self) -> Option<i64> {
        let mut robots = self.robots.clone();
        let mut seconds = 0;

        loop {
            seconds += 1;
            let mut set = VectorSet::new(self.size);
            for robot in robots.iter_mut() {
                robot.position += robot.velocity;
                set.insert(robot.position.rem_euclid(self.size));
            }
            if set.len() == robots.len() {
                break;
            }
        }

        Some(seconds)
    }
}

impl std::str::FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" v=").unwrap();
        let position = position[2..].split_once(',').unwrap();
        let velocity = velocity.split_once(',').unwrap();
        let position = Vector::new(position.0.parse().unwrap(), position.1.parse().unwrap());
        let velocity = Vector::new(velocity.0.parse().unwrap(), velocity.1.parse().unwrap());
        Ok(Robot { position, velocity })
    }
}

crate::solution::test_solution!();
