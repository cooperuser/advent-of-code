#![allow(dead_code)]

use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 6032;
pub const SAMPLE_B: i32 = 5031;

#[derive(Debug, Clone)]
enum Tile {
    Floor,
    Wall
}

#[derive(Debug, Clone)]
enum Move {
    Left,
    Right,
    Forward(i32)
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const RIGHT: usize = 0;
const DOWN_: usize = 1;
const LEFT_: usize = 2;
const UP___: usize = 3;

fn map_pos(from: usize, to: usize, pos: (i32, i32), size: i32) -> (i32, i32) {
    let (y, x) = pos;
    let s = size - 1;
    match (from, to) {
        (RIGHT, RIGHT) => (y, 0),
        (RIGHT, DOWN_) => (0, s - y),
        (RIGHT, LEFT_) => (s - y, s),
        (RIGHT, UP___) => (s, y),

        (DOWN_, RIGHT) => (s - x, 0),
        (DOWN_, DOWN_) => (0, x),
        (DOWN_, LEFT_) => (x, s),
        (DOWN_, UP___) => (s, s - x),

        (LEFT_, RIGHT) => (s - y, 0),
        (LEFT_, DOWN_) => (0, y),
        (LEFT_, LEFT_) => (y, s),
        (LEFT_, UP___) => (s, s - y),

        (UP___, RIGHT) => (x, 0),
        (UP___, DOWN_) => (0, s - x),
        (UP___, LEFT_) => (s - x, s),
        (UP___, UP___) => (s, x),
        _ => panic!()
    }
}

fn get_move(pos: (i32, i32), facing: usize, size: i32, test: bool) -> ((i32, i32), usize) {
    let face = (pos.0 / size, pos.1 / size);
    let pos_on_face = (pos.0 % size, pos.1 % size);
    if test {
        let faces = [(0, 2), (1, 0), (1, 1), (1, 2), (2, 2), (2, 3)];
        let face_num = match face {
            (0, 2) => 1,
            (1, 0) => 2,
            (1, 1) => 3,
            (1, 2) => 4,
            (2, 2) => 5,
            (2, 3) => 6,
            _ => panic!()
        };
        let dest = match (face_num, facing) {
            (1, RIGHT) => (6, LEFT_),
            (1, LEFT_) => (3, DOWN_),
            (1, UP___) => (2, DOWN_),

            (2, DOWN_) => (5, UP___),
            (2, LEFT_) => (6, UP___),
            (2, UP___) => (1, DOWN_),

            (3, DOWN_) => (5, RIGHT),
            (3, UP___) => (1, RIGHT),

            (4, RIGHT) => (6, DOWN_),

            (5, DOWN_) => (2, UP___),
            (5, LEFT_) => (3, UP___),

            (6, RIGHT) => (1, LEFT_),
            (6, DOWN_) => (2, RIGHT),
            (6, UP___) => (4, LEFT_),
            _ => panic!()
        };
        let newpos = map_pos(facing, dest.1, pos_on_face, size);
        let face = faces[dest.0 - 1];
        ((face.0 * size + newpos.0, face.1 * size + newpos.1), dest.1)
    } else {
        let faces = [(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)];
        let face_num = match face {
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 1) => 3,
            (2, 0) => 4,
            (2, 1) => 5,
            (3, 0) => 6,
            _ => panic!()
        };
        let dest = match (face_num, facing) {
            (1, LEFT_) => (4, RIGHT),
            (1, UP___) => (6, RIGHT),

            (2, RIGHT) => (5, LEFT_),
            (2, DOWN_) => (3, LEFT_),
            (2, UP___) => (6, UP___),

            (3, RIGHT) => (2, UP___),
            (3, LEFT_) => (4, DOWN_),

            (4, LEFT_) => (1, RIGHT),
            (4, UP___) => (3, RIGHT),

            (5, RIGHT) => (2, LEFT_),
            (5, DOWN_) => (6, LEFT_),

            (6, RIGHT) => (5, UP___),
            (6, DOWN_) => (2, DOWN_),
            (6, LEFT_) => (1, DOWN_),
            _ => panic!()
        };
        let newpos = map_pos(facing, dest.1, pos_on_face, size);
        let face = faces[dest.0 - 1];
        ((face.0 * size + newpos.0, face.1 * size + newpos.1), dest.1)
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: HashMap<(i32, i32), Tile>,
    insts: Vec<Move>,
    start: (i32, i32),
    bounds: (i32, i32)
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let split = raw.iter().position(|line| line.is_empty()).unwrap();
        let mut map = HashMap::new();
        let mut min = i32::MAX;
        let mut bounds = (0, 0);
        for y in 0..split {
            let line: Vec<char> = raw[y].chars().collect();
            for (x, ch) in line.iter().enumerate() {
                let pos = (y as i32, x as i32);
                match ch {
                    '#' => {
                        map.insert(pos, Tile::Wall);
                        bounds.0 = bounds.0.max(y as i32);
                        bounds.1 = bounds.1.max(x as i32);
                    },
                    '.' => {
                        if y == 0 { min = min.min(x as i32); }
                        map.insert(pos, Tile::Floor);
                        bounds.0 = bounds.0.max(y as i32);
                        bounds.1 = bounds.1.max(x as i32);
                    },
                    ' ' => {},
                    _ => panic!()
                };
            }
        }
        let mut num = 0;
        let mut insts = vec![];
        for ch in raw[split + 1].chars() {
            match ch {
                'L' => {
                    insts.push(Move::Forward(num));
                    num = 0;
                    insts.push(Move::Left);
                },
                'R' => {
                    insts.push(Move::Forward(num));
                    num = 0;
                    insts.push(Move::Right);
                },
                _ => {
                    num = 10 * num + (ch as i32 - '0' as i32)
                }
            }
        }
        insts.push(Move::Forward(num));
        Self {
            raw: raw.clone(),
            map,
            insts,
            start: (0, min),
            bounds,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut pos = self.start;
        let mut facing = 0;
        for inst in &self.insts {
            match inst {
                Move::Left => facing = (facing + 3) % 4,
                Move::Right => facing = (facing + 1) % 4,
                Move::Forward(num) => {
                    for _ in 0..*num {
                        let dir = DIRS[facing];
                        let newpos = (pos.0 + dir.0, pos.1 + dir.1);
                        match self.map.get(&newpos) {
                            Some(tile) => match tile {
                                Tile::Floor => pos = newpos,
                                Tile::Wall => break,
                            },
                            None => {
                                let mut tile: Tile = Tile::Floor;
                                let mut newpos: (i32, i32) = (0, 0);
                                match facing {
                                    0 => {
                                        for x in 0..pos.1 {
                                            newpos = (pos.0, x);
                                            if let Some(t) = self.map.get(&newpos) {
                                                tile = t.clone();
                                                break;
                                            }
                                        }
                                    },
                                    2 => {
                                        for x in (pos.1..=self.bounds.1).rev() {
                                            newpos = (pos.0, x);
                                            if let Some(t) = self.map.get(&newpos) {
                                                tile = t.clone();
                                                break;
                                            }
                                        }
                                    },
                                    1 => {
                                        for y in 0..pos.0 {
                                            newpos = (y, pos.1);
                                            if let Some(t) = self.map.get(&newpos) {
                                                tile = t.clone();
                                                break;
                                            }
                                        }
                                    },
                                    3 => {
                                        for y in (pos.0..=self.bounds.0).rev() {
                                            newpos = (y, pos.1);
                                            if let Some(t) = self.map.get(&newpos) {
                                                tile = t.clone();
                                                break;
                                            }
                                        }
                                    },
                                    _ => panic!()
                                }
                                match tile {
                                    Tile::Floor => pos = newpos,
                                    Tile::Wall => break,
                                }
                            },
                        }
                    }
                },
            }
        }
        1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing as i32
    }

    pub fn part_b(&self) -> i32 {
        let test = self.insts.len() < 20;
        let size = if test { 4 } else { 50 };
        let mut pos = self.start;
        let mut facing = 0;

        for inst in &self.insts {
            match inst {
                Move::Left => facing = (facing + 3) % 4,
                Move::Right => facing = (facing + 1) % 4,
                Move::Forward(num) => {
                    for _ in 0..*num {
                        let dir = DIRS[facing];
                        let newpos = (pos.0 + dir.0, pos.1 + dir.1);
                        match self.map.get(&newpos) {
                            Some(tile) => match tile {
                                Tile::Floor => pos = newpos,
                                Tile::Wall => break,
                            },
                            None => {
                                let out = get_move(pos, facing, size, test);
                                if let Some(tile) = self.map.get(&out.0) {
                                    match tile {
                                        Tile::Floor => {
                                            pos = out.0;
                                            facing = out.1;
                                        },
                                        Tile::Wall => break,
                                    }
                                } else { panic!() }
                            }
                        }
                    }
                },
            }
        }
        1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + facing as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
