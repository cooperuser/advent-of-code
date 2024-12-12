use std::collections::{HashSet, VecDeque};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    regions: Vec<HashSet<Vector>>,
    size: Vector,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 1930,
            answer_b: 1206,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut regions = Vec::new();
        let mut map = VectorMap::new(size);
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert(Vector::new_usize(x, y), c);
            }
        }

        let mut seen = VectorSet::new(size);
        for pos in size.iter() {
            if !seen.insert(pos).unwrap() {
                continue;
            }
            let mut deque = VecDeque::from([pos]);
            let mut region = HashSet::new();
            while let Some(pos) = deque.pop_front() {
                if !region.insert(pos) {
                    continue;
                }
                seen.insert(pos);
                let c = map.get(pos).unwrap();
                for dir in DIRS {
                    let pos = pos + dir;
                    if Some(c) == map.get(pos) {
                        deque.push_back(pos);
                    }
                }
            }
            regions.push(region);
        }

        Self {
            raw: raw.clone(),
            regions,
            size,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        for region in &self.regions {
            let area = region.len() as i64;
            let mut perimeter = 0;
            for &pos in region {
                for dir in DIRS {
                    if !region.contains(&(pos + dir)) {
                        perimeter += 1;
                    }
                }
            }
            sum += area * perimeter;
        }
        Some(sum)
    }

    fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for region in &self.regions {
            let mut edges = 0;
            for check in DIRS {
                let mut start = match check {
                    Direction::North => self.size,
                    Direction::South => Vector::new(-1, -1),
                    Direction::East => Vector::new(-1, self.size.y),
                    Direction::West => Vector::new(self.size.x, -1),
                };
                let step = check.rotate_left();
                while start.contained_in(Vector::new(-1, -1), self.size + Vector::new(1, 1)) {
                    let mut pos = start;
                    let mut edge = region.contains(&pos) && region.contains(&(pos + check));
                    if edge {
                        edges += 1;
                    }
                    while pos.contained_in(Vector::new(-1, -1), self.size + Vector::new(1, 1)) {
                        let next = pos + check;
                        if edge != (!region.contains(&pos) && region.contains(&next)) {
                            edge = !edge;
                            if edge {
                                edges += 1;
                            }
                        }
                        pos += step;
                    }
                    start += check;
                }
            }
            sum += region.len() as i64 * edges;
        }
        Some(sum)
    }
}

crate::solution::test_solution!();
