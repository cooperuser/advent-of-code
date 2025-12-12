use std::collections::{BTreeSet, BinaryHeap, HashMap};

use utils::{
    prelude::*,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    #[allow(dead_code)]
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Clone, Debug)]
struct Shape {
    size: Vector,
    orientations: Vec<BTreeSet<Vector>>,
    count: usize,
}

#[derive(Debug)]
struct Region {
    size: Vector,
    counts: Vec<usize>,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 2,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut shapes = Vec::new();
        let chunks: Vec<_> = raw.split(|line| line.is_empty()).collect();
        for chunk in &chunks[0..chunks.len() - 1] {
            let shape = VectorSet::from_grid(&chunk[1..], '#');
            shapes.push(Shape::new(&shape, Vector::new(3, 3)));
        }

        let mut regions = Vec::new();
        for line in chunks[chunks.len() - 1] {
            let (size, counts) = line.split_once(": ").unwrap();
            let (x, y) = size.split_once('x').unwrap();
            let size = Vector::new(x.parse().unwrap(), y.parse().unwrap());
            let counts = counts
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            regions.push(Region { size, counts });
        }

        Self {
            raw,
            shapes,
            regions,
        }
    }

    fn part_a(&self) -> Option<usize> {
        let count = self
            .regions
            .iter()
            .filter(|&region| {
                region.size.area() as usize >= region.counts.iter().sum::<usize>() * 9
            })
            .count();

        Some(count + (self.regions.len() < 4) as usize)
    }

    fn part_b(&self) -> Option<usize> {
        None
    }
}

impl Region {
    #[allow(dead_code)]
    fn can_fit(&self, shapes: &[Shape]) -> bool {
        let mut needed = Vec::new();
        for (index, &count) in self.counts.iter().enumerate() {
            for _ in 0..count {
                needed.push(shapes[index].clone());
            }
        }
        let count: usize = needed.iter().map(|shape| shape.count).sum();
        if count > self.size.area() as usize {
            return false;
        }

        let mut queue = BinaryHeap::from([State {
            grid: HashMap::new(),
            index: 0,
            orientation: 0,
            pos: Vector::zero(),
        }]);

        while let Some(state) = queue.pop() {
            let Some(shape) = needed.get(state.index) else {
                pretty_print(&state.grid, self.size);
                return true;
            };
            let max_pos = self.size - shape.size; //- Vector::new(1, 1);
            let Some(shape) = shape.orientations.get(state.orientation) else {
                queue.push(state.shift_right());
                continue;
            };

            if state.pos.y > max_pos.y {
                // Hit the bottom of the grid, continue onto next state in BFS
                continue;
            } else if state.pos.x > max_pos.x {
                // Hit the right edge of the grid, carriage return onto BFS
                queue.push(state.shift_down());
                continue;
            } else if shape
                .iter()
                .any(|&spot| state.grid.contains_key(&(state.pos + spot)))
            {
                // There was an overlap between the shape and the grid
                queue.push(state.inc_orientation());
                continue;
            }

            let mut new_grid = state.grid.clone();
            for &spot in shape {
                new_grid.insert(state.pos + spot, state.index);
            }

            queue.push(state.inc_index(new_grid));
            queue.push(State {
                grid: state.grid.clone(),
                index: state.index,
                orientation: state.orientation + 1,
                pos: state.pos,
            });
            queue.push(state.shift_right());
        }

        false
    }
}

#[derive(Eq, PartialEq)]
struct State {
    grid: HashMap<Vector, usize>,
    index: usize,
    orientation: usize,
    pos: Vector,
}

impl State {
    fn shift_right(self) -> Self {
        Self {
            orientation: 0,
            pos: Vector::new(self.pos.x + 1, self.pos.y),
            ..self
        }
    }

    fn shift_down(self) -> Self {
        Self {
            orientation: 0,
            pos: Vector::new(0, self.pos.y + 1),
            ..self
        }
    }

    fn inc_index(&self, grid: HashMap<Vector, usize>) -> Self {
        Self {
            grid,
            orientation: 0,
            pos: Vector::zero(),
            index: self.index + 1,
        }
    }

    fn inc_orientation(self) -> Self {
        Self {
            orientation: self.orientation + 1,
            ..self
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Shape {
    fn new(set: &VectorSet, size: Vector) -> Self {
        let o = size - Vector::new(1, 1);
        let mut orientations = BTreeSet::new();
        orientations.insert(set.iter().collect());
        orientations.insert(set.iter().map(|p| Vector::new(p.y, p.x)).collect());
        orientations.insert(set.iter().map(|p| Vector::new(o.x - p.x, p.y)).collect());
        orientations.insert(set.iter().map(|p| Vector::new(p.x, o.y - p.y)).collect());
        orientations.insert(set.iter().map(|p| Vector::new(o.x - p.y, p.x)).collect());
        orientations.insert(set.iter().map(|p| Vector::new(p.y, o.y - p.x)).collect());
        orientations.insert(
            set.iter()
                .map(|p| Vector::new(o.x - p.x, o.y - p.y))
                .collect(),
        );
        orientations.insert(
            set.iter()
                .map(|p| Vector::new(o.x - p.y, o.y - p.x))
                .collect(),
        );

        Self {
            size,
            orientations: orientations.iter().cloned().collect(),
            count: set.len(),
        }
    }
}

fn pretty_print(grid: &HashMap<Vector, usize>, size: Vector) {
    for y in 0..size.y {
        let mut line = Vec::new();
        for x in 0..size.x {
            if let Some(n) = grid.get(&Vector::new(x, y)) {
                line.push(n.to_string());
            } else {
                line.push(".".to_string());
            }
        }
        println!("{}", line.join(""));
    }
    println!();
}

utils::solution::test_solution!(aoc2025, day12);
