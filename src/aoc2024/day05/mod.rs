use std::collections::HashSet;

#[derive(Default)]
pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    rules: HashSet<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

impl Day {
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

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 143,
            answer_b: 123,
        }
    }

    fn new(raw: Vec<String>) -> Self {
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

    fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for update in &self.updates {
            if self.check_update(update) {
                sum += update[update.len() / 2]
            }
        }

        Some(sum)
    }

    fn part_b(&self) -> Option<i64> {
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
            let ordered: Vec<i64> = unordered.iter().map(|pair| pair.0).rev().collect();
            sum += ordered[update.len() / 2];
        }

        Some(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::Solution;

    #[test]
    fn part_a() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_a));
        assert_eq!(solution.part_a(), Some(meta.answer_a));
    }

    #[test]
    fn part_b() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_b));
        assert_eq!(solution.part_b(), Some(meta.answer_b));
    }
}
