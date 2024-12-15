use std::collections::{HashSet, VecDeque};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorMap<Tile>,
    start: Vector,
    size: Vector,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 94,
            answer_b: 154,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorMap::new(size);
        let mut start = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                if y == 0 && c == '.' {
                    start = Some(pos);
                }
                grid.insert(pos, c.to_string().parse().unwrap());
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            start: start.unwrap(),
            size,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut deque: VecDeque<(i64, Vector, Direction, HashSet<Vector>)> =
            VecDeque::from([(0, self.start, Direction::South, HashSet::new())]);
        let mut seen: VectorMap<i64> = VectorMap::new(self.size);
        let mut max = 0;
        while let Some((distance, pos, last_dir, mut set)) = deque.pop_front() {
            if !set.insert(pos) {
                continue;
            }
            if let Some(last) = seen.get(pos) {
                if last > distance {
                    continue;
                }
            }

            seen.insert(pos, distance);
            if pos.y == self.size.y - 1 && distance > max {
                max = distance;
                continue;
            }

            let mut next: Vec<Direction> = Vec::new();
            match self.grid.get(pos) {
                Some(Tile::Path) => {
                    for dir in DIRS {
                        if dir.flip() == last_dir {
                            continue;
                        }
                        match self.grid.get(pos + dir) {
                            None | Some(Tile::Forest) => {}
                            _ => {
                                next.push(dir);
                            }
                        }
                    }
                }
                Some(Tile::Slope(slope)) => {
                    next.push(slope);
                }
                _ => {}
            }

            if next.len() == 1 {
                deque.push_back((distance + 1, pos + next[0], next[0], set));
                continue;
            }
            for dir in next {
                deque.push_back((distance + 1, pos + dir, dir, set.clone()));
            }
        }
        Some(max)
    }

    fn part_b(&self) -> Option<i64> {
        let mut deque: VecDeque<(i64, Vector, Direction, HashSet<Vector>)> =
            VecDeque::from([(0, self.start, Direction::South, HashSet::new())]);
        let mut seen: VectorMap<i64> = VectorMap::new(self.size);
        let mut max = 0;
        while let Some((distance, pos, last_dir, mut set)) = deque.pop_front() {
            if !set.insert(pos) {
                continue;
            }
            if let Some(last) = seen.get(pos) {
                if last > distance {
                    continue;
                }
            }

            seen.insert(pos, distance);
            if pos.y == self.size.y - 1 && distance > max {
                max = distance;
                continue;
            }

            let mut next: Vec<Direction> = Vec::new();
            if let Some(Tile::Path) | Some(Tile::Slope(_)) = self.grid.get(pos) {
                for dir in DIRS {
                    if dir.flip() == last_dir {
                        continue;
                    }
                    match self.grid.get(pos + dir) {
                        None | Some(Tile::Forest) => {}
                        _ => {
                            next.push(dir);
                        }
                    }
                }
            }

            if next.len() == 1 {
                deque.push_back((distance + 1, pos + next[0], next[0], set));
                continue;
            }
            for dir in next {
                deque.push_back((distance + 1, pos + dir, dir, set.clone()));
            }
        }
        Some(max)
    }
}

impl std::str::FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            "^" => Ok(Self::Slope(Direction::North)),
            "v" => Ok(Self::Slope(Direction::South)),
            ">" => Ok(Self::Slope(Direction::East)),
            "<" => Ok(Self::Slope(Direction::West)),
            _ => Err(()),
        }
    }
}

crate::solution::test_solution!();
