use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    buyers: Vec<i64>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample_a.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 37327623,
            answer_b: 23,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self {
            raw: raw.clone(),
            buyers: raw.iter().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut output = Vec::new();
        for &buyer in &self.buyers {
            let mut secret = buyer;
            for _ in 0..2000 {
                secret = Self::prune(Self::mix(secret, secret * 64));
                secret = Self::prune(Self::mix(secret, secret / 32));
                secret = Self::prune(Self::mix(secret, secret * 2048));
            }
            output.push(secret);
        }
        Some(output.iter().sum())
    }

    fn part_b(&self) -> Option<i64> {
        let mut sequences: HashSet<VecDeque<i64>> = HashSet::new();
        let mut maps: Vec<HashMap<VecDeque<i64>, i64>> = Vec::new();
        for &buyer in &self.buyers {
            let mut map = HashMap::new();
            let mut secret = buyer;
            let mut sequence = VecDeque::from([0, 0, 0, 0]);
            let mut last = 0;
            for i in 0..2000 {
                secret = Self::prune(Self::mix(secret, secret * 64));
                secret = Self::prune(Self::mix(secret, secret / 32));
                secret = Self::prune(Self::mix(secret, secret * 2048));

                let ones = secret % 10;
                let diff = ones - last;
                last = ones;
                sequence.pop_front();
                sequence.push_back(diff);

                sequences.insert(sequence.clone());
                if i < 3 || map.contains_key(&sequence) {
                    continue;
                }
                map.insert(sequence.clone(), ones);
            }
            maps.push(map);
        }

        let mut max = 0;
        for target in sequences {
            let mut output = Vec::new();
            for map in &maps {
                if let Some(ones) = map.get(&target) {
                    output.push(*ones);
                }
            }
            max = max.max(output.iter().sum());
        }
        Some(max)
    }
}

impl Day {
    fn mix(secret: i64, given: i64) -> i64 {
        secret ^ given
    }

    fn prune(value: i64) -> i64 {
        value % 16777216
    }
}

crate::solution::test_solution!();
