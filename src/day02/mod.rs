pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 2;
pub const ANSWER_B: i64 = 4;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    reports: Vec<Vec<i64>>,
}

fn remove_level(report: &[i64]) -> Vec<Vec<i64>> {
    let mut reports = vec![];
    for index in 0..report.len() {
        let mut report = report.to_owned();
        report.remove(index);
        reports.push(report);
    }
    reports
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            reports: raw
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut safe = 0;
        'outer: for report in &self.reports {
            let mut last_level = report[0];
            let increasing = report[1] - report[0] > 0;
            for level in report.iter().skip(1) {
                let diff = level - last_level;
                last_level = *level;
                if diff == 0 || (diff > 0) != increasing || diff.abs() > 3 {
                    continue 'outer;
                }
            }
            safe += 1;
        }
        Some(safe)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut safe = 0;
        for r in 0..self.reports.len() {
            'outer: for report in remove_level(&self.reports[r]) {
                let mut last_level = report[0];
                let increasing = report[1] - report[0] > 0;
                for level in report.iter().skip(1) {
                    let diff = level - last_level;
                    last_level = *level;
                    if diff == 0 || (diff > 0) != increasing || diff.abs() > 3 {
                        continue 'outer;
                    }
                }
                safe += 1;
                break 'outer;
            }
        }
        Some(safe)
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
