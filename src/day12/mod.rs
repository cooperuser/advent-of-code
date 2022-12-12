#![allow(dead_code)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 31;
pub const SAMPLE_B: i32 = 29;

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Default)]
pub struct Solution {
    start: (i32, i32),
    end: (i32, i32),
    grid: Vec<Vec<u8>>,
    size: (i32, i32),
    #[allow(dead_code)]
    raw: Vec<String>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut grid: Vec<Vec<u8>> = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in raw.iter().enumerate() {
            let mut row: Vec<u8> = vec![];
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        start = (y as i32, x as i32);
                        row.push(b'a' - b'a');
                    },
                    'E' => {
                        end = (y as i32, x as i32);
                        row.push(b'z' - b'a');
                    },
                    c => row.push(c as u8 - b'a')
                }
            }
            grid.push(row);
        }
        Self {
            raw: raw.clone(),
            start,
            end,
            size: (grid.len() as i32, grid[0].len() as i32),
            grid,
            ..Default::default()
        }
    }

    fn find_path(&self, start: (i32, i32)) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut stack: Vec<((i32, i32), usize)> = vec![(start, 0)];

        while !stack.is_empty() {
            let (pos, length) = stack.remove(0);
            if visited.contains(&pos) { continue }
            visited.insert(pos);
            if pos == self.end { return length as i32; }
            let a = self.grid[pos.0 as usize][pos.1 as usize];
            for dir in DIRS {
                let new = (pos.0 + dir.0, pos.1 + dir.1);
                if new.0 < 0 || new.0 >= self.size.0 { continue }
                if new.1 < 0 || new.1 >= self.size.1 { continue }

                let b = self.grid[new.0 as usize][new.1 as usize];
                if b <= a + 1 { stack.push((new, length + 1)); }
            }
        }
        0
    }

    pub fn part_a(&self) -> i32 {
        self.find_path(self.start)
    }

    pub fn part_b(&self) -> i32 {
        let mut starts = vec![];
        let mut lengths = vec![];

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if self.grid[y as usize][x as usize] == 0 {
                    starts.push((y, x));
                }
            }
        }

        for start in starts {
            let length = self.find_path(start);
            if length != 0 { lengths.push(length); }
        }

        *lengths.iter().min().unwrap() as i32
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
