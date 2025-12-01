use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    rotations: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3,
            answer_b: 6,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut rotations = Vec::new();
        for line in &raw {
            let (sign, value) = line.split_at(1);
            let value: i64 = value.parse().unwrap();
            rotations.push(value * if sign == "R" { 1 } else { -1 });
        }
        Self { raw, rotations }
    }

    fn part_a(&self) -> Option<i64> {
        let mut dial = 50;
        let mut count = 0;

        for &rotation in &self.rotations {
            dial += rotation;
            if dial.rem_euclid(100) == 0 {
                count += 1;
            }
        }

        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        let mut dial: i64 = 50;
        let mut count = 0;

        for &rotation in &self.rotations {
            let next = dial.rem_euclid(100) + rotation;

            if dial.rem_euclid(100) == 0 {
                count += rotation.abs() / 100;
            } else if next <= 0 {
                count += 1 + next.abs() / 100;
            } else if next >= 100 {
                count += next.abs() / 100;
            }

            dial += rotation;
        }

        Some(count)
    }
}

utils::solution::test_solution!(aoc2025, day01);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r1000() {
        let input = ["R1000"];
        let solution = Day::new(input.iter().map(|&s| s.into()).collect());
        assert_eq!(solution.part_b(), Some(10));
    }

    #[test]
    fn r50() {
        let input = ["R50", "R100"];
        let solution = Day::new(input.iter().map(|&s| s.into()).collect());
        assert_eq!(solution.part_b(), Some(2));
    }

    #[test]
    fn l1000() {
        let input = ["R50", "L1000"];
        let solution = Day::new(input.iter().map(|&s| s.into()).collect());
        assert_eq!(solution.part_b(), Some(11));
    }

    #[test]
    fn r50_l1_r1() {
        let input = ["R50", "L1", "R1"];
        let solution = Day::new(input.iter().map(|&s| s.into()).collect());
        assert_eq!(solution.part_b(), Some(2));
    }
}
