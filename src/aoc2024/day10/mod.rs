use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    direction::DIRS,
    vector::{Vector, VectorMap},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    map: VectorMap<i64>,
    zeros: Vec<Vector>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 36,
            answer_b: 81,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut map: VectorMap<i64> = VectorMap::new(size);
        let mut zeros = Vec::new();
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                map.insert(pos, c as i64 - '0' as i64);
                if c == '0' {
                    zeros.push(pos);
                }
            }
        }

        Self {
            raw: raw.clone(),
            map,
            zeros,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut trails = 0;
        for &pos in &self.zeros {
            let mut nines: HashSet<Vector> = HashSet::new();
            let mut deque: VecDeque<Vector> = VecDeque::new();
            deque.push_front(pos);

            while let Some(pos) = deque.pop_back() {
                let height = self.map.get(pos).unwrap();
                if height == 9 {
                    nines.insert(pos);
                    continue;
                }

                for dir in DIRS {
                    let next_pos = pos + dir;
                    match self.map.get(next_pos) {
                        Some(next) if next == height + 1 => deque.push_front(next_pos),
                        _ => {}
                    }
                }
            }

            trails += nines.len();
        }

        Some(trails as i64)
    }

    fn part_b(&self) -> Option<i64> {
        let mut trails = 0;
        for &pos in &self.zeros {
            let mut nines = 0;
            let mut deque: VecDeque<Vector> = VecDeque::new();
            deque.push_front(pos);

            while let Some(pos) = deque.pop_back() {
                let height = self.map.get(pos).unwrap();
                if height == 9 {
                    nines += 1;
                    continue;
                }

                for dir in DIRS {
                    let next_pos = pos + dir;
                    match self.map.get(next_pos) {
                        Some(next) if next == height + 1 => deque.push_front(next_pos),
                        _ => {}
                    }
                }
            }

            trails += nines;
        }

        Some(trails as i64)
    }
}

crate::solution::test_solution!();
