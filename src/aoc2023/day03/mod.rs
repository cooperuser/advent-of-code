use crate::vector::{Vector, VectorMap};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    parts: VectorMap<Vec<i64>>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 4361,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut numbers: VectorMap<i64> = VectorMap::new(size);
        let mut symbols: VectorMap<Vec<i64>> = VectorMap::new(size);

        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let pos = Vector::new_usize(x, y);
                if c.is_ascii_digit() {
                    numbers.insert(pos, c as i64 - b'0' as i64);
                } else {
                    symbols.insert(pos, Vec::new());
                }
            }
        }

        for y in 0..size.y {
            let mut x = 0;
            while x < size.x {
                let mut pos = Vector::new(x, y);
                let mut number = 0;
                let mut length = 0;
                while let Some(num) = numbers.get(pos) {
                    number = number * 10 + num;
                    pos.x += 1;
                    length += 1;
                }

                if length != 0 {
                    for y in y - 1..=y + 1 {
                        for x in x - 1..=x + length {
                            let pos = Vector::new(x, y);
                            if let Some(symbol) = symbols.get_mut(pos) {
                                symbol.push(number);
                            }
                        }
                    }
                }

                x += length + 1;
            }
        }

        Self {
            raw: raw.clone(),
            parts: symbols,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for (_, numbers) in self.parts.iter() {
            sum += numbers.iter().sum::<i64>();
        }

        Some(sum)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

crate::solution::test_solution!();
