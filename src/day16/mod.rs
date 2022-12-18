use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 1651;
pub const SAMPLE_B: i32 = 0;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    valves: HashMap<String, i32>,
    tunnels: HashMap<String, HashSet<String>>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut valves = HashMap::new();
        let mut tunnels = HashMap::new();
        for line in &raw {
            let words: Vec<&str> = line.split_whitespace().collect();
            let valve = words[1].to_string();
            let rate: i32 = words[4].split(['=', ';']).collect::<Vec<&str>>()[1].parse().unwrap();
            let leads: HashSet<String> = words[9..].join("").split(',').map(|s| s.to_string()).collect();
            valves.insert(valve.clone(), rate);
            tunnels.insert(valve.clone(), leads);

        }
        Self {
            raw: raw.clone(),
            valves,
            tunnels,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        println!("{:?}", self.valves);
        println!("{:?}", self.tunnels);
        let mut opened: HashMap<String, i32> = HashMap::new();
        let mut room = "AA".to_string();
        let mut timer = 1;

        while timer < 30 {
            println!("{room}");
            let rooms = self.tunnels.get(&room).unwrap();
            let mut max_rate = (0, "");
            for r in rooms {
                if opened.contains_key(r) { continue; }
                let rate = *self.valves.get(r).unwrap();
                if rate > max_rate.0 { max_rate = (rate, r); }
            }
            opened.insert(room.clone(), timer);
            room = max_rate.1.to_string();
            timer += 1;
        }

        opened
            .iter()
            .fold(0, |pressure, (valve, time)| {
                let rate = self.valves.get(valve).unwrap();
                pressure + rate * (30 - time)
            })
    }

    pub fn part_b(&self) -> i32 {
        0
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
