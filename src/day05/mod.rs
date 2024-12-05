use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 143;
pub const ANSWER_B: i64 = 123;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    rules: HashSet<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let (rules, updates) = raw.split_once(|line| line.is_empty()).unwrap();
        let rules = rules
            .iter()
            .map(|line| line.split_once("|").unwrap())
            .map(|rule| (rule.0.parse().unwrap(), rule.1.parse().unwrap()))
            .collect();
        let updates = updates
            .iter()
            .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
            .collect();

        Self {
            raw: raw.clone(),
            rules,
            updates,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for update in &self.updates {
            if self.check_update(update) {
                sum += update[update.len() / 2]
            }
        }

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        let updates: Vec<Vec<i64>> = self
            .updates
            .iter()
            .filter(|update| !self.check_update(update))
            .cloned()
            .collect();

        for update in &updates {
            let mut unordered: Vec<(i64, usize)> = Vec::new();
            for &a in update {
                let mut count = 0;
                for &b in update {
                    if self.rules.contains(&(a, b)) {
                        count += 1;
                    }
                }
                unordered.push((a, count));
            }

            unordered.sort_by_key(|num| num.1);
            let ordered: Vec<i64> = unordered.iter().map(|pair| pair.0).collect();
            sum += ordered[update.len() / 2];
        }

        Some(sum)
    }

    fn check_update(&self, update: &[i64]) -> bool {
        for (i, &a) in update.iter().enumerate() {
            for &b in update.iter().take(i) {
                if self.rules.contains(&(a, b)) {
                    return false;
                }
            }

            for &b in update.iter().skip(i + 1) {
                if !self.rules.contains(&(a, b)) {
                    return false;
                }
            }
        }

        true
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
