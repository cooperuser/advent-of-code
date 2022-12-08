#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 21;
pub const SAMPLE_B: i32 = 8;

#[derive(Default, Debug)]
struct Visibility {
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

impl Visibility {
    fn is_visible(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    size: usize,
    trees: Vec<Vec<i32>>,
    visibility: Vec<Vec<Visibility>>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let size = raw[0].len();
        let mut trees: Vec<Vec<i32>> = vec![];
        for line in &raw {
            trees.push(line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect());
        }
        let mut visibility = vec![];
        for _ in 0..size {
            let mut line = vec![];
            for _ in 0..size { line.push(Visibility::default()); }
            visibility.push(line);
        }

        for i in 0..size {
            visibility[0][i].up = true;
            visibility[size - 1][i].down = true;
            visibility[i][0].left = true;
            visibility[i][size - 1].right = true;
        }

        for y in 0..size {
            for x in 0..size {
                let tree = trees[y][x];
                let mut up = vec![];
                let mut down = vec![];
                let mut left = vec![];
                let mut right = vec![];

                for i in 0..y { up.push(trees[i][x]) }
                for i in 0..x { left.push(trees[y][i]) }
                for i in y + 1..size { down.push(trees[i][x]) }
                for i in x + 1..size { right.push(trees[y][i]) }

                if *up.iter().max().unwrap_or(&0) < tree { visibility[y][x].up = true; }
                if *down.iter().max().unwrap_or(&0) < tree { visibility[y][x].down = true; }
                if *left.iter().max().unwrap_or(&0) < tree { visibility[y][x].left = true; }
                if *right.iter().max().unwrap_or(&0) < tree { visibility[y][x].right = true; }
            }
        }

        Self {
            raw: raw.clone(),
            size,
            trees,
            visibility,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut visible = 0;
        for line in &self.visibility {
            for tree in line {
                if tree.is_visible() {
                    visible += 1;
                }
            }
        }
        visible
    }

    pub fn part_b(&self) -> i32 {
        let mut score = 0;
        for y in 0..self.size {
            for x in 0..self.size {
                let tree = self.trees[y][x];
                let mut up = 0;
                let mut down = 0;
                let mut left = 0;
                let mut right = 0;

                for i in (0..y).rev() {
                    up += 1;
                    if self.trees[i][x] >= tree { break }
                }
                for i in (0..x).rev() {
                    left += 1;
                    if self.trees[y][i] >= tree { break }
                }
                for i in y + 1..self.size {
                    down += 1;
                    if self.trees[i][x] >= tree { break }
                }
                for i in x + 1..self.size {
                    right += 1;
                    if self.trees[y][i] >= tree { break }
                }

                score = score.max(up * down * left * right);
            }
        }
        score
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
