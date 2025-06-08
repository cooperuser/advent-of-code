use std::rc::Rc;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    locks: Vec<Vec<usize>>,
    keys: Vec<Vec<usize>>,
    height: usize,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let blocks = raw.split(|line| line.is_empty()).map(|block| {
            block
                .iter()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        });
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        let mut height: Option<usize> = None;
        for block in blocks {
            height = Some(block.len());
            if block[0][0] == '.' {
                let mut key = Vec::new();
                for col in 0..block[0].len() {
                    for (row, line) in block.iter().enumerate().rev() {
                        if line[col] == '.' {
                            key.push(block.len() - row - 1);
                            break;
                        }
                    }
                }
                keys.push(key);
            } else {
                let mut lock = Vec::new();
                for col in 0..block[0].len() {
                    for (row, line) in block.iter().enumerate() {
                        if line[col] == '.' {
                            lock.push(row - 1);
                            break;
                        }
                    }
                }
                locks.push(lock);
            }
        }
        Self {
            raw: raw.clone(),
            locks,
            keys,
            height: height.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut combos = 0;
        for lock in &self.locks {
            'key: for key in &self.keys {
                for (lock, key) in lock.iter().zip(key) {
                    if lock + key >= self.height {
                        continue 'key;
                    }
                }
                combos += 1;
            }
        }
        Some(combos)
    }

    fn part_b(&self) -> Option<i64> {
        Some(0)
    }
}

crate::solution::test_solution!();
