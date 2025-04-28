pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    lines: Vec<Vec<char>>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample_a.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
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
        let mut sum = 0;

        for line in &self.lines {
            let mut nums: Vec<u32> = Vec::new();
            let mut three: Vec<String> = Vec::new();
            let mut four: Vec<String> = Vec::new();
            let mut five: Vec<String> = Vec::new();

            for c in line {
                if let Some(num) = c.to_digit(10) {
                    nums.push(num);
                    continue;
                }

                three.push(c.to_string());
                four.push(c.to_string());
                five.push(c.to_string());

                if three.len() > 3 {
                    three.remove(0);
                }
                if four.len() > 4 {
                    four.remove(0);
                }
                if five.len() > 5 {
                    five.remove(0);
                }

                match three.join("").as_str() {
                    "one" => nums.push(1),
                    "two" => nums.push(2),
                    "six" => nums.push(6),
                    _ => (),
                }
                match four.join("").as_str() {
                    "four" => nums.push(4),
                    "five" => nums.push(5),
                    "nine" => nums.push(9),
                    _ => (),
                }
                match five.join("").as_str() {
                    "three" => nums.push(3),
                    "seven" => nums.push(7),
                    "eight" => nums.push(8),
                    _ => (),
                }
            }

            let a = nums.first().unwrap();
            let b = nums.last().unwrap();
            sum += a * 10 + b;
        }

        Some(sum as i64)
    }
}

crate::solution::test_solution!();
