use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    columns: Vec<Column>,
    numbers: Vec<Vec<Option<i64>>>,
}

#[derive(Clone)]
enum Operation {
    Plus,
    Star,
}

#[derive(Clone)]
struct Column {
    operation: Operation,
    rows: Vec<i64>,
}

impl Column {
    fn compute(&self) -> i64 {
        match self.operation {
            Operation::Plus => self.rows.iter().sum(),
            Operation::Star => self.rows.iter().product(),
        }
    }
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 4277556,
            answer_b: 3263827,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut columns = Vec::new();

        let numbers: Vec<Vec<Option<i64>>> = raw
            .iter()
            .take(raw.len() - 1)
            .map(|line| {
                line.chars()
                    .map(|c| match c != ' ' {
                        true => Some(c.to_string().parse::<i64>().unwrap()),
                        false => None,
                    })
                    .collect()
            })
            .collect();

        for op in raw.last().unwrap().split_whitespace() {
            let op = match op {
                "+" => Operation::Plus,
                _ => Operation::Star,
            };

            columns.push(Column {
                operation: op,
                rows: Vec::new(),
            })
        }

        Self {
            raw,
            columns,
            numbers,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut columns = self.columns.clone();

        for line in self.raw.iter().take(self.raw.len() - 1) {
            let cs = line.split_whitespace();
            for (column, number) in cs.enumerate() {
                columns[column].rows.push(number.parse().unwrap());
            }
        }

        Some(columns.iter().map(Column::compute).sum())
    }

    fn part_b(&self) -> Option<i64> {
        let mut columns = self.columns.clone();

        let mut column = 0;
        for c in 0..self.numbers[0].len() {
            let mut number = 0;
            for row in &self.numbers {
                if let Some(n) = row[c] {
                    number = number * 10 + n;
                }
            }

            match number != 0 {
                true => columns[column].rows.push(number),
                false => column += 1,
            }
        }

        Some(columns.iter().map(Column::compute).sum())
    }
}

utils::solution::test_solution!(aoc2025, day06);
