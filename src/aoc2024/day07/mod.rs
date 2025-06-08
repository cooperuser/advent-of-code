use std::{collections::VecDeque, rc::Rc};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    equations: Vec<Equation>,
}

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Day {
    fn helper(&self, equation: &Equation, concatenate: bool) -> i64 {
        let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
        deque.push_back((equation.numbers[0], 1));
        while let Some(partial) = deque.pop_front() {
            if partial.1 == equation.numbers.len() {
                if partial.0 == equation.target {
                    return equation.target;
                }
                continue;
            }

            deque.push_back((partial.0 + equation.numbers[partial.1], partial.1 + 1));
            deque.push_back((partial.0 * equation.numbers[partial.1], partial.1 + 1));
            if concatenate {
                let number = equation.numbers[partial.1];
                let magnitude = (number as f64).log10().floor() as u32 + 1;
                deque.push_back((partial.0 * 10i64.pow(magnitude) + number, partial.1 + 1));
            }
        }

        0
    }
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3749,
            answer_b: 11387,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut equations = Vec::new();
        for line in raw.iter() {
            let (left, right) = line.split_once(':').unwrap();
            equations.push(Equation {
                target: left.parse().unwrap(),
                numbers: right
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            });
        }
        Self {
            raw: raw.clone(),
            equations,
        }
    }

    fn part_a(&self) -> Option<i64> {
        Some(self.equations.iter().map(|eq| self.helper(eq, false)).sum())
    }

    fn part_b(&self) -> Option<i64> {
        Some(self.equations.iter().map(|eq| self.helper(eq, true)).sum())
    }
}

crate::solution::test_solution!();
