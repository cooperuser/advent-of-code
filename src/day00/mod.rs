use std::rc::Rc;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 0,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self { raw: raw.clone() }
    }

    fn part_a(&self) -> Option<i64> {
        None
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

crate::solution::test_solution!();
