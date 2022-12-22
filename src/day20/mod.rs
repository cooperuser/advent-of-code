#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 3;
pub const SAMPLE_B: i64 = 1623178306;

fn shuffle(numbers: &mut Vec<(i64, i64)>) {
    let len = numbers.len() as i64;
    for i in 0..len {
        let index = numbers.iter().position(|n| n.0 == i).unwrap() as i64;
        let number = numbers[index as usize].1;
        let mut dest = (index + number) % (len - 1);
        if dest < 0 { dest += len - 1 }

        let val = numbers.remove(index as usize);
        numbers.insert(dest as usize, val);
    }
}

fn get_coords(numbers: &Vec<(i64, i64)>) -> i64 {
    let index = numbers.iter().position(|n| n.1 == 0).unwrap();
    let a = numbers[(index + 1000) % numbers.len()].1;
    let b = numbers[(index + 2000) % numbers.len()].1;
    let c = numbers[(index + 3000) % numbers.len()].1;
    a + b + c
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    numbers: Vec<(i64, i64)>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut numbers: Vec<(i64, i64)> = vec![];
        for (index, line) in raw.iter().enumerate() {
            numbers.push((index as i64, line.parse().unwrap()))
        }
        Self {
            raw: raw.clone(),
            numbers,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i64 {
        let mut numbers = self.numbers.clone();
        shuffle(&mut numbers);
        get_coords(&numbers)
    }

    pub fn part_b(&self) -> i64 {
        let key = 811589153;
        let mut numbers: Vec<(i64, i64)> = self.numbers.iter().map(|(i, n)| (*i, n * key)).collect();
        for _ in 0..10 { shuffle(&mut numbers); }
        get_coords(&numbers)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
