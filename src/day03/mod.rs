#![allow(dead_code)]

use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 4361;
pub const ANSWER_B: i64 = 467835;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    symbols: HashMap<(i64, i64), char>,
    numbers: HashMap<(i64, i64), (i64, usize)>,
}

fn debug(soln: &Solution) {
    for y in 0..soln.raw.len() {
        let mut range = 0..soln.raw.len();
        while let Some(x) = range.next() {
            let pos = (x as i64, y as i64);
            match soln.numbers.get(&pos) {
                Some((num, len)) => {
                    print!("{num}");
                    for _ in 0..len-1 {
                        range.next();
                    }
                },
                None => {
                    match soln.symbols.get(&pos) {
                        Some(s) => print!("{s}"),
                        None => print!("."),
                    }
                }
            }
        }
        println!();
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut symbols = HashMap::new();
        let mut numbers = HashMap::new();
        for (y, line) in raw.iter().enumerate() {
            let mut iter = line.chars();
            let mut x = 0;
            while let Some(c) = iter.next() {

                if c.is_ascii_digit() {
                    let mut num = c.to_digit(10).unwrap() as i64;
                    let mut len = 1;
                    let spot = x;
                    loop {
                        match iter.next() {
                            Some(d) => {
                                if !d.is_ascii_digit() {
                                    numbers.insert((spot as i64, y as i64), (num, len));
                                    x += 1;
                                    if d != '.' {
                                        symbols.insert((x as i64, y as i64), d);
                                    }
                                    break;
                                }
                                num = num * 10 + d.to_digit(10).unwrap() as i64;
                                len += 1;
                            },
                            None => {
                                numbers.insert((spot as i64, y as i64), (num, len));
                                break;
                            },
                        }
                        x += 1;
                    }
                } else if c != '.' {
                    symbols.insert((x as i64, y as i64), c);
                }
                x += 1;
            }
        }
        Self {
            raw: raw.clone(),
            symbols,
            numbers,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        'next: for ((x, y), (num, len)) in &self.numbers {
            let len = *len as i64;
            for i in x-1..x+len+1 {
                for j in y-1..y+2 {
                    if self.symbols.contains_key(&(i, j)) {
                        sum += num;
                        continue 'next;
                    }
                }
            }
        }
        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        let max = self.numbers
            .iter()
            .max_by_key(|(_, (_, len))| len)
            .unwrap().1.1 as i64;
        for (pos, symbol) in &self.symbols {
            if *symbol != '*' { continue; }
            let mut found = Vec::new();
            for y in pos.1-1..pos.1+2 {
                for x in pos.0-max..pos.0+max {
                    if let Some((num, len)) = self.numbers.get(&(x, y)) {
                        if pos.0 >= x - 1 && pos.0 <= x + *len as i64 {
                            found.push(num);
                        }
                    }
                }
            }
            if found.len() >= 2 {
                sum += found[0] * found[1];
            }
        }
        Some(sum)
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
