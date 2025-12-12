use std::collections::{BTreeSet, HashSet, VecDeque};

use utils::{
    prelude::*,
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
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
        let mut count = 0;
        for (i, region) in self.regions.iter().enumerate() {
            if region.can_fit(&self.shapes) {
                println!("yes");
                count += 1;
            }
            println!("{}/{}", i + 1, self.regions.len());
        }
        // let count: usize = self
        //     .regions
        //     .iter()
        //     .filter(|r| r.can_fit(&self.shapes))
        //     .count();
        Some(count)
    }

    fn part_b(&self) -> Option<usize> {
        None
    }
}

impl Region {
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

        let mut queue = VecDeque::from([(VectorMap::new(self.size), 0, 0, Vector::zero())]);
        while let Some((grid, index, orientation, pos)) = queue.pop_front() {
            pretty_print(&grid, self.size);
            let Some(shape) = needed.get(index) else {
                return true;
            };
            let max_pos = self.size - shape.size; //- Vector::new(1, 1);
            let Some(shape) = shape.orientations.get(orientation) else {
                queue.push_back((grid, index, 0, Vector::new(pos.x + 1, pos.y)));
                continue;
            };

            if pos.y > max_pos.y {
                // Hit the bottom of the grid, continue onto next state in BFS
                continue;
            } else if pos.x > max_pos.x {
                // Hit the right edge of the grid, carriage return onto BFS
                queue.push_back((grid, index, orientation, Vector::new(0, pos.y + 1)));
                continue;
            } else if shape.iter().any(|&spot| grid.contains(pos + spot)) {
                // There was an overlap between the shape and the grid
                queue.push_back((grid, index, orientation + 1, pos));
                continue;
            }

            let mut new_grid = grid.clone();
            for &spot in shape {
                new_grid.insert(pos + spot, index);
            }
            // pretty_print(&new_grid, self.size);
            queue.push_back((new_grid, index + 1, 0, Vector::zero()));
            queue.push_back((grid.clone(), index, orientation + 1, pos));
            queue.push_back((grid, index, orientation, Vector::new(pos.x + 1, pos.y)));
        }

        false
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

    // fn print(&self) {
    //     for o in &self.orientations {
    //         for y in 0..self.size.y {
    //             let mut line = Vec::new();
    //             for x in 0..self.size.x {
    //                 let pos = Vector::new(x, y);
    //                 if o.contains(&pos) {
    //                     line.push("#");
    //                 } else {
    //                     line.push(".");
    //                 }
    //             }
    //             println!("{}", line.join(""));
    //         }
    //         println!();
    //     }
    // }
}

fn pretty_print(grid: &VectorMap<usize>, size: Vector) {
    for y in 0..size.y {
        let mut line = Vec::new();
        for x in 0..size.x {
            if let Some(n) = grid.get(Vector::new(x, y)) {
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
