use crate::prelude::*;
use std::{collections::HashMap, fmt::Display};

use crate::vector::Vector;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    codes: Vec<Vec<Number>>,
    numberpad: HashMap<(Number, Number), Vec<Input>>,
    arrowpad: HashMap<(Input, Input), Vec<Input>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Number {
    Number(usize),
    Enter,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Input {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 126384,
            answer_b: 154115708116294,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut codes = Vec::new();
        for line in raw.iter() {
            let mut input = Vec::new();
            for c in line.chars() {
                input.push(match c {
                    '0'..='9' => Number::Number(c as usize - '0' as usize),
                    'A' => Number::Enter,
                    _ => panic!(),
                });
            }
            codes.push(input);
        }

        let mut numberpad = HashMap::new();
        let mut numbers = Vec::new();
        numbers.push(Number::Enter);
        numbers.extend((0..=9).map(Number::Number));

        for &start in &numbers {
            let start_pos = Self::key_to_pos(start);
            for &end in &numbers {
                let end_pos = Self::key_to_pos(end);
                numberpad.insert(
                    (start, end),
                    Self::get_path(start_pos, end_pos, Vector::new(0, 3)),
                );
            }
        }

        let mut arrowpad = HashMap::new();
        let arrows = [
            Input::Enter,
            Input::Up,
            Input::Down,
            Input::Left,
            Input::Right,
        ];

        for &start in &arrows {
            let start_pos = Self::arrow_to_pos(start);
            for &end in &arrows {
                let end_pos = Self::arrow_to_pos(end);
                arrowpad.insert(
                    (start, end),
                    Self::get_path(start_pos, end_pos, Vector::new(0, 0)),
                );
            }
        }

        Self {
            raw: raw.clone(),
            codes,
            numberpad,
            arrowpad,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        for code in &self.codes {
            let length = self.get_code_length(code, 2);
            let number = Self::get_code_number(code);
            sum += length * number;
        }
        Some(sum)
    }

    fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for code in &self.codes {
            let length = self.get_code_length(code, 25);
            let number = Self::get_code_number(code);
            sum += length * number;
        }
        Some(sum)
    }
}

impl Day {
    fn get_code_length(&self, code: &[Number], arrowpads: usize) -> i64 {
        let mut inputs: HashMap<(Input, Input), i64> = HashMap::new();

        let mut number = Number::Enter;
        for &key in code {
            let mut arrow = Input::Enter;
            for &input in self.numberpad.get(&(number, key)).unwrap() {
                *inputs.entry((arrow, input)).or_default() += 1;
                arrow = input;
            }
            number = key;
        }

        for _ in 0..arrowpads {
            let mut next: HashMap<(Input, Input), i64> = HashMap::new();
            for ((a, b), count) in inputs {
                let mut arrow = Input::Enter;
                for &input in self.arrowpad.get(&(a, b)).unwrap() {
                    *next.entry((arrow, input)).or_default() += count;
                    arrow = input;
                }
            }
            inputs = next;
        }

        inputs.values().sum()
    }

    fn get_code_number(code: &[Number]) -> i64 {
        let mut number = 0;
        for (i, &key) in code[0..code.len() - 1].iter().enumerate() {
            let Number::Number(n) = key else {
                continue;
            };
            number += 10i64.pow((code.len() - i - 2) as u32) * n as i64;
        }
        number
    }

    fn key_to_pos(value: Number) -> Vector {
        match value {
            Number::Enter => Vector::new(2, 3),
            Number::Number(0) => Vector::new(1, 3),
            Number::Number(n) => Vector::new_usize((n - 1) % 3, 2 - (n - 1) / 3),
        }
    }

    fn arrow_to_pos(value: Input) -> Vector {
        match value {
            Input::Up => Vector::new(1, 0),
            Input::Down => Vector::new(1, 1),
            Input::Left => Vector::new(0, 1),
            Input::Right => Vector::new(2, 1),
            Input::Enter => Vector::new(2, 0),
        }
    }

    fn get_path(start: Vector, end: Vector, blank: Vector) -> Vec<Input> {
        let mut path = Vec::new();
        let diff = end - start;
        let vertical: Vec<Input> = [
            [Input::Down].repeat(diff.y.max(0) as usize),
            [Input::Up].repeat((-diff.y).max(0) as usize),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();
        let horizontal: Vec<Input> = [
            [Input::Right].repeat(diff.x.max(0) as usize),
            [Input::Left].repeat((-diff.x).max(0) as usize),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();

        if diff.x > 0 && Vector::new(start.x, end.y) != blank {
            path.extend(vertical);
            path.extend(horizontal);
        } else if Vector::new(end.x, start.y) != blank {
            path.extend(horizontal);
            path.extend(vertical);
        } else if Vector::new(start.x, end.y) != blank {
            path.extend(vertical);
            path.extend(horizontal);
        }

        path.push(Input::Enter);
        path
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Input::Up => "^",
                Input::Down => "v",
                Input::Left => "<",
                Input::Right => ">",
                Input::Enter => "A",
            }
        )
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::Number(n) => format!("{}", n),
                Number::Enter => "A".to_string(),
            }
        )
    }
}

crate::solution::test_solution!();

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::Solution;

    #[test]
    fn a() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.input));
        let code = [
            Number::Number(0),
            Number::Number(2),
            Number::Number(9),
            Number::Enter,
        ];
        let answer = solution.get_code_length(&code, 2);
        assert_eq!(68, answer);
    }

    #[test]
    fn b() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.input));
        let code = [
            Number::Number(9),
            Number::Number(8),
            Number::Number(0),
            Number::Enter,
        ];
        let answer = solution.get_code_length(&code, 2);
        assert_eq!(60, answer);
    }

    #[test]
    fn c() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.input));
        let code = [
            Number::Number(1),
            Number::Number(7),
            Number::Number(9),
            Number::Enter,
        ];
        let answer = solution.get_code_length(&code, 2);
        assert_eq!(68, answer);
    }

    #[test]
    fn d() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.input));
        let code = [
            Number::Number(4),
            Number::Number(5),
            Number::Number(6),
            Number::Enter,
        ];
        let answer = solution.get_code_length(&code, 2);
        assert_eq!(64, answer);
    }

    #[test]
    fn e() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.input));
        let code = [
            Number::Number(3),
            Number::Number(7),
            Number::Number(9),
            Number::Enter,
        ];
        let answer = solution.get_code_length(&code, 2);
        assert_eq!(64, answer);
    }
}
