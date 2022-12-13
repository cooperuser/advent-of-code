use std::cmp::Ordering;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 13;
pub const SAMPLE_B: i32 = 140;

#[derive(Debug, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>)
}

impl From<String> for Packet {
    fn from(str: String) -> Self {
        let chars: Vec<char> = str.chars().collect();
        if chars[0] != '[' { return Packet::Num(str.parse::<i32>().unwrap()) }
        let mut parts = vec![];
        let mut depth = 0;
        let mut part = vec![];

        for &char in chars[1..chars.len() - 1].iter() {
            match char {
                ',' => {
                    if depth == 0 {
                        parts.push(Packet::from(part.join("")));
                        part.clear();
                        continue;
                    }
                },
                '[' => { depth += 1; },
                ']' => { depth -= 1; },
                _ => {}
            }
            part.push(char.to_string());
        }
        if !part.is_empty() {
            parts.push(Packet::from(part.join("")));
        }

        Packet::List(parts)
    }
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::Num(n) => n.to_string(),
            Packet::List(vec) => format!("[{}]", vec.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")),
        }
    }
}

impl Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("compare {} with {}", self.to_string(), other.to_string());
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => {
                for i in 0..left.len() {
                    if i == right.len() { return Ordering::Greater }
                    let check = left[i].cmp(&right[i]);
                    if check != Ordering::Equal { return check }
                }
                if left.len() < right.len() { return Ordering::Less }
                Ordering::Equal
            }
            (Packet::Num(left), Packet::Num(right)) => {
                if left < right { Ordering::Less }
                else if left > right { Ordering::Greater }
                else { Ordering::Equal }
            }
            (Packet::List(_left), Packet::Num(_right)) => {
                // println!("  convert {} to [{}]", right.to_string(), right.to_string());
                self.cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::Num(_left), Packet::List(_right)) => {
                // println!("  convert {} to [{}]", left.to_string(), left.to_string());
                Packet::List(vec![self.clone()]).cmp(other)
            }
        }
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    pairs: Vec<(Packet, Packet)>,
    packets: Vec<Packet>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut pairs = vec![];
        let mut packets = vec![Packet::from("[[2]]".to_string()), Packet::from("[[6]]".to_string())];
        for group in raw.split(|line| line.is_empty()) {
            if group.is_empty() { continue }
            let (left, right) = (group.get(0).unwrap(), group.get(1).unwrap());
            let (left, right) = (Packet::from(left.clone()), Packet::from(right.clone()));
            pairs.push((left.clone(), right.clone()));
            packets.push(left);
            packets.push(right);
        }
        Self {
            raw: raw.clone(),
            pairs,
            packets,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut sum = 0;
        for (index, (left, right)) in self.pairs.iter().enumerate() {
            if left.cmp(right) == Ordering::Less {
                sum += index as i32 + 1;
            }
        }
        sum
    }

    pub fn part_b(&self) -> i32 {
        let mut packets = self.packets.clone();
        packets.sort_by(|a, b| a.cmp(b));
        let a = packets.iter().position(|p| p.to_string() == "[[2]]").unwrap() + 1;
        let b = packets.iter().position(|p| p.to_string() == "[[6]]").unwrap() + 1;
        a as i32 * b as i32
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
