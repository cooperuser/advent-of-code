use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 19114;
pub const ANSWER_B: i64 = 167409079868000;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    workflows: HashMap<String, Vec<Workflow>>,
    ratings: Vec<[i64; 4]>,
}

#[derive(Debug)]
struct Workflow {
    operation: Operation,
    action: Action,
}

#[derive(Debug)]
enum Operation {
    LessThan(usize, i64),
    GreaterThan(usize, i64),
    Conditionless,
}

#[derive(Debug)]
enum Action {
    Label(String),
    Reject,
    Accept,
}

fn get_category(c: &str) -> usize {
    match c {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("unknown variable {c}"),
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let blocks: Vec<_> = raw.split(|line| line.is_empty()).collect();
        let mut workflows = HashMap::new();

        for line in blocks[0] {
            let mut workflow = Vec::new();
            let (label, clauses) = line.split_once('{').unwrap();
            for clause in clauses.strip_suffix('}').unwrap().split(',') {
                let (operation, action) = match clause.split_once(':') {
                    Some((rule, action)) => {
                        let operation = if let Some((c, v)) = rule.split_once('<') {
                            Operation::LessThan(get_category(c), v.parse().unwrap())
                        } else if let Some((c, v)) = rule.split_once('>') {
                            Operation::GreaterThan(get_category(c), v.parse().unwrap())
                        } else {
                            panic!("unknown operation: {rule}")
                        };
                        (operation, action)
                    }
                    None => (Operation::Conditionless, clause),
                };

                workflow.push(Workflow {
                    operation,
                    action: match action {
                        "R" => Action::Reject,
                        "A" => Action::Accept,
                        _ => Action::Label(action.to_string()),
                    }
                })
            }
            workflows.insert(label.to_string(), workflow);
        }

        let mut ratings = Vec::new();
        for line in blocks[1] {
            let mut rating = [0i64; 4];
            let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            let parts: Vec<_> = line.split(',').collect();
            for part in parts {
                let (category, value) = part.split_once('=').unwrap();
                let category = get_category(category);
                let value: i64 = value.parse().unwrap();
                rating[category] = value;
            }
            ratings.push(rating);
        }

        Self {
            raw: raw.clone(),
            workflows,
            ratings,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let sum = self
            .ratings
            .iter()
            .filter(|rating| self.test_rating(rating))
            .map(|rating| rating.iter().sum::<i64>())
            .sum::<i64>();

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        None
    }

    fn test_rating(&self, rating: &[i64; 4]) -> bool {
        let mut label = "in".to_string();
        'outer: loop {
            let workflow = self.workflows.get(&label).unwrap();
            for Workflow { operation, action } in workflow {
                let success = match operation {
                    Operation::LessThan(c, v) => rating[*c] < *v,
                    Operation::GreaterThan(c, v) => rating[*c] > *v,
                    Operation::Conditionless => true,
                };
                if success {
                    match action {
                        Action::Label(l) => label = l.clone(),
                        Action::Reject => break 'outer false,
                        Action::Accept => break 'outer true,
                    }
                    break;
                }
            }
        }
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
