use crate::direction::Direction;
use crate::vector::{Vector, VectorMap, VectorSet};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
}

impl Day {
    fn compute_path(&self, extra: Option<Vector>) -> Option<VectorMap<Vec<bool>>> {
        let mut seen: VectorMap<Vec<bool>> = VectorMap::new(self.size);
        let mut facing = Direction::North;
        let mut pos = self.start;
        loop {
            match seen.get(pos) {
                Some(dirs) => {
                    if dirs[facing as usize] {
                        return None;
                    }
                }
                None => {
                    seen.insert(pos, vec![false; 4]);
                }
            }

            seen.get_mut(pos).unwrap()[facing as usize] = true;
            let next = pos + facing;
            if !next.contained_in(Vector::zero(), self.size) {
                return Some(seen);
            } else if self.grid.contains(next) || Some(next) == extra {
                facing = facing.rotate_right();
            } else {
                pos = next;
            }
        }
    }
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 41,
            answer_b: 6,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorSet::new(size);
        let mut start: Option<Vector> = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = Vector::new_usize(x, y);
                if c == '#' {
                    grid.insert(spot);
                } else if c == '^' {
                    start = Some(spot);
                }
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        Some(self.compute_path(None).unwrap().len() as i64)
    }

    fn part_b(&self) -> Option<i64> {
        let mut total = 0;
        for (spot, _) in self.compute_path(None).unwrap() {
            if spot == self.start {
                continue;
            } else if self.compute_path(Some(spot)).is_none() {
                total += 1;
            }
        }

        Some(total)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::Solution;

    #[test]
    fn part_a() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_a));
        assert_eq!(solution.part_a(), Some(meta.answer_a));
    }

    #[test]
    fn part_b() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_b));
        assert_eq!(solution.part_b(), Some(meta.answer_b));
    }
}
