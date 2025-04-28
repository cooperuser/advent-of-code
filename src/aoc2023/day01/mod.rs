pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    lines: Vec<Vec<char>>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 142,
            answer_b: 281,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            lines: raw.iter().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for line in &self.lines {
            let a = *line.iter().find(|c| c.is_numeric()).unwrap() as u8;
            let b = *line.iter().rev().find(|c| c.is_numeric()).unwrap() as u8;
            sum += ((a - b'0') * 10 + (b - b'0')) as i64;
        }

        Some(sum)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

crate::solution::test_solution!();
