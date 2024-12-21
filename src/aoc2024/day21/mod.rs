use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    codes: Vec<Vec<Key>>,
    keypad: HashMap<(Key, Key), Vec<Input>>,
    arrowpad: HashMap<(Input, Input), Vec<Input>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Key {
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

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 126384,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let mut codes = Vec::new();
        for line in raw.iter() {
            let mut input = Vec::new();
            for c in line.chars() {
                input.push(match c {
                    '0'..='9' => Key::Number(c as usize - '0' as usize),
                    'A' => Key::Enter,
                    _ => panic!(),
                });
            }
            codes.push(input);
        }

        let keypad_size = Vector::new(3, 4);
        let mut raw_keypad: VectorMap<Key> = VectorMap::new(keypad_size);
        raw_keypad.insert(Vector::new(0, 0), Key::Number(7));
        raw_keypad.insert(Vector::new(1, 0), Key::Number(8));
        raw_keypad.insert(Vector::new(2, 0), Key::Number(9));
        raw_keypad.insert(Vector::new(0, 1), Key::Number(4));
        raw_keypad.insert(Vector::new(1, 1), Key::Number(5));
        raw_keypad.insert(Vector::new(2, 1), Key::Number(6));
        raw_keypad.insert(Vector::new(0, 2), Key::Number(1));
        raw_keypad.insert(Vector::new(1, 2), Key::Number(2));
        raw_keypad.insert(Vector::new(2, 2), Key::Number(3));
        raw_keypad.insert(Vector::new(1, 3), Key::Number(0));
        raw_keypad.insert(Vector::new(2, 3), Key::Enter);

        let mut keypad = HashMap::new();
        let mut keys = Vec::new();
        keys.push(Key::Enter);
        keys.extend((0..=9).map(Key::Number));

        for &start in &keys {
            let start_pos = Self::key_to_pos(start);
            for &end in &keys {
                let end_pos = Self::key_to_pos(end);
                let mut deque: VecDeque<(Vector, Vec<Input>)> =
                    VecDeque::from([(start_pos, Vec::new())]);
                let mut visited = VectorSet::new(keypad_size);
                while let Some((pos, mut path)) = deque.pop_front() {
                    if !pos.contained_in(Vector::zero(), keypad_size)
                        || !visited.insert(pos).unwrap()
                        || !raw_keypad.contains(pos)
                    {
                        continue;
                    }

                    if pos == end_pos {
                        path.push(Input::Enter);
                        keypad.insert((start, end), path);
                        break;
                    }

                    for dir in DIRS {
                        let mut path = path.clone();
                        path.push(dir.into());
                        deque.push_back((pos + dir, path));
                    }
                }
            }
        }

        let arrowpad_size = Vector::new(3, 2);
        let mut raw_arrows: VectorMap<Input> = VectorMap::new(arrowpad_size);
        raw_arrows.insert(Vector::new(1, 0), Input::Up);
        raw_arrows.insert(Vector::new(2, 0), Input::Enter);
        raw_arrows.insert(Vector::new(0, 1), Input::Left);
        raw_arrows.insert(Vector::new(1, 1), Input::Down);
        raw_arrows.insert(Vector::new(2, 1), Input::Right);

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
                let mut deque: VecDeque<(Vector, Vec<Input>)> =
                    VecDeque::from([(start_pos, Vec::new())]);
                let mut visited = VectorSet::new(arrowpad_size);
                while let Some((pos, mut path)) = deque.pop_front() {
                    if !pos.contained_in(Vector::zero(), arrowpad_size)
                        || !visited.insert(pos).unwrap()
                        || !raw_arrows.contains(pos)
                    {
                        continue;
                    }

                    if pos == end_pos {
                        path.push(Input::Enter);
                        arrowpad.insert((start, end), path);
                        break;
                    }

                    for dir in DIRS {
                        let mut path = path.clone();
                        path.push(dir.into());
                        deque.push_back((pos + dir, path));
                    }
                }
            }
        }

        Self {
            raw: raw.clone(),
            codes,
            keypad,
            arrowpad,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut number_pos = Key::Enter;
        let mut arrow_a = Input::Enter;
        let mut arrow_b = Input::Enter;
        let mut counts: Vec<i64> = Vec::new();
        for code in &self.codes {
            let mut inputs: Vec<Input> = Vec::new();
            for &key in code {
                for &arrow in self.keypad.get(&(number_pos, key)).unwrap() {
                    for &arrow in self.arrowpad.get(&(arrow_a, arrow)).unwrap() {
                        inputs.extend(self.arrowpad.get(&(arrow_b, arrow)).unwrap());
                        arrow_b = arrow;
                    }
                    arrow_a = arrow;
                }
                number_pos = key;
            }
            let mut number = 0;
            for (i, &key) in code[0..code.len() - 1].iter().enumerate() {
                let Key::Number(n) = key else {
                    continue;
                };
                number += 10i64.pow((code.len() - i - 2) as u32) * n as i64;
            }
            counts.push(inputs.len() as i64 * number);
            println!(
                "{}: {}",
                code.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                inputs
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
            println!("{:?}", inputs.len());
        }
        Some(counts.iter().sum())
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl Day {
    fn key_to_pos(value: Key) -> Vector {
        match value {
            Key::Enter => Vector::new(2, 3),
            Key::Number(0) => Vector::new(1, 3),
            Key::Number(n) => Vector::new_usize((n - 1) % 3, 2 - (n - 1) / 3),
        }
    }

    fn pos_to_key(value: Vector) -> Key {
        match (value.x, value.y) {
            (0, 0) => Key::Number(7),
            (1, 0) => Key::Number(8),
            (2, 0) => Key::Number(9),
            (0, 1) => Key::Number(4),
            (1, 1) => Key::Number(5),
            (2, 1) => Key::Number(6),
            (0, 2) => Key::Number(1),
            (1, 2) => Key::Number(2),
            (2, 2) => Key::Number(3),
            (1, 3) => Key::Number(0),
            (2, 3) => Key::Enter,
            _ => panic!(),
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

    fn pos_to_arrow(value: Vector) -> Input {
        match (value.x, value.y) {
            (1, 0) => Input::Up,
            (2, 0) => Input::Enter,
            (0, 1) => Input::Left,
            (1, 1) => Input::Down,
            (2, 1) => Input::Right,
            _ => panic!(),
        }
    }
}

impl From<Direction> for Input {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self::Up,
            Direction::South => Self::Down,
            Direction::East => Self::Right,
            Direction::West => Self::Left,
        }
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

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Number(n) => format!("{}", n),
                Key::Enter => "A".to_string(),
            }
        )
    }
}

crate::solution::test_solution!();
