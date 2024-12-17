use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

type Node = HashMap<Direction, (i64, Vector, Direction, HashSet<Vector>)>;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    graph: HashMap<Vector, Node>,
    size: Vector,
    start: Vector,
    end: Vector,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7036,
            answer_b: 45,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorSet::new(size);
        let mut start: Option<Vector> = None;
        let mut end: Option<Vector> = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    continue;
                }
                let pos = Vector::new_usize(x, y);
                if c == 'S' {
                    start = Some(pos);
                } else if c == 'E' {
                    end = Some(pos);
                }
                grid.insert(pos);
            }
        }

        let mut graph: HashMap<Vector, Node> = HashMap::new();
        let mut nodes = VectorSet::new(size);
        nodes.insert(start.unwrap());
        nodes.insert(end.unwrap());
        for pos in grid.iter() {
            let mut dirs = Vec::new();
            for dir in DIRS {
                if grid.contains(pos + dir) {
                    dirs.push(dir);
                }
            }
            if dirs.len() > 2 {
                nodes.insert(pos);
            }
        }

        for start in nodes.iter() {
            'outgoing: for outgoing in DIRS {
                if !grid.contains(start + outgoing) {
                    continue;
                }

                let mut pos = start + outgoing;
                let mut facing = outgoing;
                let mut score = 1;
                let mut set = HashSet::new();
                while !nodes.contains(pos) {
                    set.insert(pos);
                    let left = facing.rotate_left();
                    let right = facing.rotate_right();
                    if grid.contains(pos + facing) {
                        pos += facing;
                        score += 1;
                        continue;
                    } else if grid.contains(pos + left) {
                        facing = left;
                        pos += facing;
                        score += 1001;
                        continue;
                    } else if grid.contains(pos + right) {
                        facing = right;
                        pos += facing;
                        score += 1001;
                        continue;
                    }
                    continue 'outgoing;
                }

                graph
                    .entry(start)
                    .or_default()
                    .insert(outgoing, (score, pos, facing, set));
            }
        }

        Self {
            raw: raw.clone(),
            graph,
            size,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut min = i64::MAX;
        let mut visited: VectorMap<i64> = VectorMap::new(self.size);
        let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
            score: 0,
            position: self.start,
            direction: Direction::East,
            path: HashSet::new(),
        }]);
        while let Some(State {
            score,
            position,
            direction,
            path: _,
        }) = heap.pop()
        {
            match visited.get(position) {
                Some(s) if s > score => continue,
                _ => {}
            }
            visited.insert(position, score);
            if position == self.end {
                if score < min {
                    min = score;
                }
                continue;
            }

            for (&dir, node) in self.graph.get(&position).unwrap() {
                if !visited.contains(node.1) {
                    let s = if dir == direction { 0 } else { 1000 };
                    heap.push(State {
                        score: score + node.0 + s,
                        position: node.1,
                        direction: node.2,
                        path: HashSet::new(),
                    })
                }
            }
        }

        Some(min)
    }

    fn part_b(&self) -> Option<i64> {
        let mut min: Option<i64> = None;
        let mut visited: HashMap<(Vector, Direction), i64> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
            score: 0,
            position: self.start,
            direction: Direction::East,
            path: HashSet::new(),
        }]);
        let mut set = VectorSet::new(self.size);
        let mut paths: HashSet<(Vector, Direction)> = HashSet::new();
        set.insert(self.start);

        while let Some(State {
            score,
            position,
            direction,
            path,
        }) = heap.pop()
        {
            if position == self.end {
                if min.is_none() || Some(score) == min {
                    min = Some(score);
                    paths.extend(path);
                } else {
                    break;
                }
                continue;
            }

            if min.is_some() && score > min? {
                continue;
            }

            match visited.get(&(position, direction)) {
                Some(s) if *s < score => continue,
                _ => {}
            }
            visited.insert((position, direction), score);

            for (&dir, node) in self.graph.get(&position).unwrap() {
                if !path.contains(&(node.1, node.2)) {
                    let s = if dir == direction { 0 } else { 1000 };
                    let mut path = path.clone();
                    path.insert((position, dir));
                    heap.push(State {
                        score: score + node.0 + s,
                        position: node.1,
                        direction: node.2,
                        path,
                    })
                }
            }
        }

        let mut set = VectorSet::new(self.size);
        set.insert(self.end);
        for (position, direction) in paths {
            set.insert(position);
            let junction = self.graph.get(&position).unwrap();
            for &spot in &junction.get(&direction).unwrap().3 {
                set.insert(spot);
            }
        }

        Some(set.len() as i64)
    }
}

#[derive(Debug)]
struct State {
    score: i64,
    position: Vector,
    direction: Direction,
    path: HashSet<(Vector, Direction)>,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for State {}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

crate::solution::test_solution!();
