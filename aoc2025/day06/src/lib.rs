use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    columns: Vec<Column>,
    columns2: Vec<Column>,
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Star,
}

#[derive(Debug, Clone)]
struct Column {
    operation: Operation,
    rows: Vec<i64>,
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

        let numbers: Vec<Vec<_>> = raw
            .iter()
            .take(raw.len() - 1)
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == ' ' {
                            None
                        } else {
                            Some(c.to_string().parse::<i64>().unwrap())
                        }
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

        let mut columns2 = columns.clone();

        for line in raw.iter().take(raw.len() - 1) {
            let cs = line.split_whitespace();
            for (column, number) in cs.enumerate() {
                columns[column].rows.push(number.parse().unwrap());
            }
        }

        let mut column = 0;
        for x in 0..raw[0].len() {
            let mut number = 0;
            for y in 0..raw.len() - 1 {
                if let Some(n) = numbers[y][x] {
                    number = number * 10 + n;
                }
            }

            if number != 0 {
                columns2[column].rows.push(number);
            }

            if number == 0 || x == raw[0].len() - 1 {
                column += 1;
            }
        }

        Self {
            raw,
            columns,
            columns2,
        }
    }

    fn part_a(&self) -> Option<i64> {
        Some(
            self.columns
                .iter()
                .map(|column| match column.operation {
                    Operation::Plus => column.rows.iter().sum::<i64>(),
                    Operation::Star => column.rows.iter().product(),
                })
                .sum(),
        )
    }

    fn part_b(&self) -> Option<i64> {
        Some(
            self.columns2
                .iter()
                .map(|column| match column.operation {
                    Operation::Plus => column.rows.iter().sum::<i64>(),
                    Operation::Star => column.rows.iter().product(),
                })
                .sum(),
        )
    }
}

utils::solution::test_solution!(aoc2025, day06);
