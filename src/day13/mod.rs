use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 405;
pub const ANSWER_B: i64 = 400;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
struct Pattern {
    points: HashSet<(i64, i64)>,
    size: (i64, i64),
    rows: Vec<i64>,
    cols: Vec<i64>,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum RowCol {
    Row(i64),
    Col(i64),
}

impl Pattern {
    fn debug(&self, spot: (i64, i64), slice_row: Option<&i64>, slice_col: Option<&i64>) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(col) = slice_col {
            lines.push(format!("{}><{}", " ".repeat(*col as usize), " ".repeat((self.size.1 - *col - 1) as usize)));
        } else {
            lines.push(" ".repeat(self.size.1 as usize + 1));
        }
        for row in 0..self.size.0 {
            let mut line = "".to_string();
            if Some(&(row + 1)) == slice_row {
                line = format!("{}v", line);
            } else if Some(&row) == slice_row {
                line = format!("{}^", line);
            } else {
                line = format!("{} ", line);
            }
            for col in 0..self.size.1 {
                if (row, col) == spot {
                    line = format!("{}O", line);
                } else if self.points.contains(&(row, col)) {
                    line = format!("{}#", line);
                } else {
                    line = format!("{}.", line);
                }
            }
            lines.push(line);
        }
        lines
    }
}

fn count_row(points: &HashSet<(i64, i64)>, row: i64, max: i64) -> i64 {
    let mut value = 0;

    for col in 0..max {
        if points.contains(&(row, col)) {
            value |= 1 << col;
        }
    }

    value
}

fn count_col(points: &HashSet<(i64, i64)>, col: i64, max: i64) -> i64 {
    let mut value = 0;

    for row in 0..max {
        if points.contains(&(row, col)) {
            value |= 1 << row;
        }
    }

    value
}

fn find_lines(nums: &Vec<i64>) -> HashSet<i64> {
    let mut found = HashSet::new();

    'slice: for slice in 1..nums.len() {
        for offset in 0..nums.len() {
            let left = (slice - offset - 1) as i64;
            let right = slice + offset;

            if left < 0 || right >= nums.len() {
                break;
            }

            if nums[left as usize] != nums[right] {
                continue 'slice;
            }
        }
        found.insert(slice as i64);
    }

    found
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut patterns = Vec::new();

        for group in raw.split(|line| line.is_empty()) {
            let mut points = HashSet::new();
            for (row, line) in group.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        points.insert((row as i64, col as i64));
                    }
                }
            }

            let size = (group.len() as i64, group[0].len() as i64);
            patterns.push(Pattern {
                rows: (0..size.0).map(|row| count_row(&points, row, size.1)).collect(),
                cols: (0..size.1).map(|col| count_col(&points, col, size.0)).collect(),
                points,
                size,
            });
        }

        Self {
            raw: raw.clone(),
            patterns,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0i64;

        for pattern in &self.patterns {
            let rows = find_lines(&pattern.rows);
            let cols = find_lines(&pattern.cols);
            sum += rows.iter().sum::<i64>() * 100 + cols.iter().sum::<i64>();
        }

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;

        'pattern: for pattern in &self.patterns {
            let rows_orig: HashSet<RowCol> = find_lines(&pattern.rows).iter().map(|r| RowCol::Row(*r)).collect();
            let cols_orig: HashSet<RowCol> = find_lines(&pattern.cols).iter().map(|c| RowCol::Col(*c)).collect();
            let orig: HashSet<RowCol> = rows_orig.union(&cols_orig).cloned().collect();

            for row in 0..pattern.size.0 {
                for col in 0..pattern.size.1 {
                    let mut rows = pattern.rows.clone();
                    let mut cols = pattern.cols.clone();

                    rows[row as usize] ^= 1 << col;
                    cols[col as usize] ^= 1 << row;

                    let rows: HashSet<RowCol> = find_lines(&rows).iter().map(|r| RowCol::Row(*r)).collect();
                    let cols: HashSet<RowCol> = find_lines(&cols).iter().map(|r| RowCol::Col(*r)).collect();
                    let slices: HashSet<RowCol> = rows.union(&cols).cloned().collect();

                    for slice in slices.difference(&orig) {
                        match slice {
                            RowCol::Row(r) => {
                                sum += r * 100;
                                continue 'pattern;
                            },
                            RowCol::Col(c) => {
                                sum += c;
                                continue 'pattern;
                            },
                        }
                    }
                }
            }
        }

        Some(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
