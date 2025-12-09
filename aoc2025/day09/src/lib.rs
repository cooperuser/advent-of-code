use utils::{
    direction::{Direction, Edge},
    prelude::*,
    vector::Vector,
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    tiles: Vec<Vector>,
    segments: Vec<(Vector, Vector, Edge)>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 50,
            answer_b: 24,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut tiles = Vec::new();
        for line in &raw {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            tiles.push(Vector::new(x, y));
        }

        let segments: Vec<_> = tiles
            .windows(2)
            .map(|window| (window[0], window[1]))
            .chain([(*tiles.last().unwrap(), *tiles.first().unwrap())])
            .map(|(a, b)| (a, b, Direction::try_from(b - a).unwrap().to_edge()))
            .collect();

        Self {
            raw,
            tiles,
            segments,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut max = 0;

        for (i, &a) in self.tiles.iter().enumerate() {
            for &b in self.tiles.iter().skip(i + 1) {
                let diff = (b - a).abs() + Vector::new(1, 1);
                max = max.max(diff.area());
            }
        }

        Some(max)
    }

    fn part_b(&self) -> Option<i64> {
        let mut max = 0;

        for (i, &a) in self.tiles.iter().enumerate() {
            for &b in self.tiles.iter().skip(i + 1) {
                if self.is_in_path(a, b) {
                    let diff = (b - a).abs() + Vector::new(1, 1);
                    max = max.max(diff.area());
                }
            }
        }

        Some(max)
    }
}

impl Day {
    fn is_in_path(&self, a: Vector, b: Vector) -> bool {
        let min = a.min(b);
        let max = a.max(b);

        for &(a, b, edge) in &self.segments {
            match edge {
                Edge::Vertical => {
                    if a.x > min.x && a.x < max.x {
                        let min_y = a.y.min(b.y);
                        let max_y = a.y.max(b.y);
                        if min.y.max(min_y) < max.y.min(max_y) {
                            return false;
                        }
                    }
                }
                Edge::Horizontal => {
                    if a.y > min.y && a.y < max.y {
                        let min_x = a.x.min(b.x);
                        let max_x = a.x.max(b.x);
                        if min.x.max(min_x) < max.x.min(max_x) {
                            return false;
                        }
                    }
                }
            }
        }

        // Count intersections
        let center = (max + min) / 2;
        let mut intersections = 0;
        for &(a, b, edge) in &self.segments {
            if edge == Edge::Horizontal || a.x <= center.x {
                continue;
            }

            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);
            if center.y > min_y && center.y < max_y {
                intersections += 1;
            }
        }

        intersections % 2 == 1
    }
}

utils::solution::test_solution!(aoc2025, day09);
