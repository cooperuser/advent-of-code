use utils::{
    prelude::*,
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    map: VectorSet,
    size: Vector,
    start: i64,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 21,
            answer_b: 40,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let map = VectorSet::from_grid(&raw, '^');
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let start = raw[0].chars().position(|c| c == 'S').unwrap() as i64;

        Self {
            raw,
            map,
            size,
            start,
        }
    }

    fn part_a(&self) -> Option<usize> {
        let mut beams = VectorSet::new(self.size);
        beams.insert(Vector::new(self.start, 0));
        let mut count = 0;

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if !beams.contains(Vector::new(x, y)) {
                    continue;
                }

                if self.map.contains(Vector::new(x, y)) {
                    beams.insert(Vector::new(x - 1, y + 1));
                    beams.insert(Vector::new(x + 1, y + 1));
                    count += 1;
                } else {
                    beams.insert(Vector::new(x, y + 1));
                }
            }
        }

        Some(count)
    }

    fn part_b(&self) -> Option<usize> {
        let mut beams = VectorMap::new(self.size);
        beams.insert(Vector::new(self.start, 0), 1);

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let Some(c) = beams.get(Vector::new(x, y)) else {
                    continue;
                };

                if self.map.contains(Vector::new(x, y)) {
                    let pos = Vector::new(x - 1, y + 1);
                    beams.insert(pos, beams.get(pos).unwrap_or_default() + c);
                    let pos = Vector::new(x + 1, y + 1);
                    beams.insert(pos, beams.get(pos).unwrap_or_default() + c);
                } else {
                    let pos = Vector::new(x, y + 1);
                    beams.insert(pos, beams.get(pos).unwrap_or_default() + c);
                }
            }
        }

        Some(
            (0..self.size.x)
                .map(|x| {
                    beams
                        .get(Vector::new(x, self.size.y - 1))
                        .unwrap_or_default()
                })
                .sum(),
        )
    }
}

utils::solution::test_solution!(aoc2025, day08);
