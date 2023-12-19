use std::{collections::HashMap, str::FromStr};

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
    ratings: Vec<HashMap<Category, i64>>,
}

enum Workflow {
    Target(Target),
    Workflow {
        category: Category,
        operation: Operation,
        predicate: i64,
        target: Target,
    },
}

#[derive(PartialEq, Eq, Hash)]
enum Category {
    Extremely,
    Musical,
    Aerodynamic,
    Shiny,
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::Extremely),
            "m" => Ok(Self::Musical),
            "a" => Ok(Self::Aerodynamic),
            "s" => Ok(Self::Shiny),
            _ => Err(format!("unknown char {s}")),
        }
    }
}

enum Operation {
    LessThan,
    GreaterThan,
}

enum Target {
    Label(String),
    Reject,
    Accept,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let blocks: Vec<_> = raw.split(|line| line.is_empty()).collect();
        let mut workflows = HashMap::new();
        for line in blocks[0] {
            let mut workflow = Vec::new();
            let (label, parts_raw) = line.split_once('{').unwrap();
            for part in parts_raw.strip_suffix('}').unwrap().split(',') {
                match part.split_once(':') {
                    Some((rule, target)) => {
                        let operation;
                        let category;
                        let value;
                        if let Some((c, v)) = rule.split_once('<') {
                            operation = Some(Operation::LessThan);
                            category = c.parse().ok();
                            value = v.parse().ok();
                        } else if let Some((c, v)) = rule.split_once('>') {
                            operation = Some(Operation::GreaterThan);
                            category = c.parse().ok();
                            value = v.parse().ok();
                        } else {
                            panic!();
                        }
                        let target = match target {
                            "R" => Target::Reject,
                            "A" => Target::Accept,
                            _ => Target::Label(target.to_string()),
                        };

                        workflow.push(Workflow::Workflow {
                            category: category.unwrap(),
                            operation: operation.unwrap(),
                            predicate: value.unwrap(),
                            target,
                        })
                    }
                    None => {
                        workflow.push(Workflow::Target(match part {
                            "R" => Target::Reject,
                            "A" => Target::Accept,
                            _ => Target::Label(part.to_string()),
                        }));
                    }
                }
            }
            workflows.insert(label.to_string(), workflow);
        }

        let mut ratings = Vec::new();
        for line in blocks[1] {
            let mut rating = HashMap::new();
            let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            let parts: Vec<_> = line.split(',').collect();
            for part in parts {
                let (category, value) = part.split_once('=').unwrap();
                let category: Category = category.parse().unwrap();
                let value: i64 = value.parse().unwrap();
                rating.insert(category, value);
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
            .map(|rating| rating.values().sum::<i64>())
            .sum::<i64>();

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for x in 1..=4000 {
            for m in 1..=4000 {
                println!("x: {x}, m: {m}, sum: {sum}");
                for a in 1..=4000 {
                    for s in 1..=4000 {
                        let rating = HashMap::from([
                            (Category::Extremely, x),
                            (Category::Musical, m),
                            (Category::Aerodynamic, a),
                            (Category::Shiny, s),
                        ]);
                        if self.test_rating(&rating) {
                            sum += x + m + a + s;
                        }
                    }
                }
            }
        }
        Some(sum)
    }

    fn test_rating(&self, rating: &HashMap<Category, i64>) -> bool {
        let mut label = "in".to_string();
        'outer: loop {
            let workflow = self.workflows.get(&label).unwrap();
            for step in workflow {
                match step {
                    Workflow::Workflow {
                        category,
                        operation,
                        predicate,
                        target,
                    } => {
                        let value = rating.get(category).unwrap();
                        let success = match operation {
                            Operation::LessThan => value < predicate,
                            Operation::GreaterThan => value > predicate,
                        };
                        if success {
                            match target {
                                Target::Label(target) => label = target.clone(),
                                Target::Reject => break 'outer false,
                                Target::Accept => break 'outer true,
                            }
                            break;
                        }
                    }
                    Workflow::Target(target) => {
                        match target {
                            Target::Label(target) => label = target.clone(),
                            Target::Reject => break 'outer false,
                            Target::Accept => break 'outer true,
                        }
                        break;
                    }
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
