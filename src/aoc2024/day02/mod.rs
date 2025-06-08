use crate::prelude::*;

#[derive(Default)]
pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    reports: Vec<Vec<i64>>,
}

impl Day {
    fn remove_level(report: &[i64]) -> Vec<Vec<i64>> {
        let mut reports = vec![];
        for index in 0..report.len() {
            let mut report = report.to_owned();
            report.remove(index);
            reports.push(report);
        }
        reports
    }
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 2,
            answer_b: 4,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
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

    fn part_a(&self) -> Option<i64> {
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

    fn part_b(&self) -> Option<i64> {
        let mut safe = 0;
        for r in 0..self.reports.len() {
            'outer: for report in Self::remove_level(&self.reports[r]) {
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

crate::solution::test_solution!();
