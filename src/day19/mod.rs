#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 33;
pub const SAMPLE_B: i32 = 62;

#[derive(Debug)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32)
}

impl Blueprint {
    fn get(&self, time: usize, materials: (i32, i32, i32), bots: (i32, i32, i32, i32)) -> i32 {
        if time > 18 { return 0 }
        if materials.0 < 0 || materials.1 < 0 || materials.2 < 0 { return 0 }
        let materials = (materials.0 + bots.0, materials.1 + bots.1, materials.2 + bots.2);
        let geodes = bots.3;
        let build_none = self.get(
            time + 1,
            materials,
            bots
        );
        let build_ore = self.get(
            time + 1,
            (materials.0 - self.ore, materials.1, materials.2),
            (bots.0 + 1, bots.1, bots.2, bots.3)
        );
        let build_clay = self.get(
            time + 1,
            (materials.0 - self.clay, materials.1, materials.2),
            (bots.0, bots.1 + 1, bots.2, bots.3)
        );
        let build_obsidian = self.get(
            time + 1,
            (materials.0 - self.obsidian.0, materials.1 - self.obsidian.1, materials.2),
            (bots.0, bots.1, bots.2 + 1, bots.3)
        );
        let build_geode = self.get(
            time + 1,
            (materials.0 - self.geode.0, materials.1, materials.2 - self.geode.1),
            (bots.0, bots.1, bots.2, bots.3 + 1)
        );
        geodes + build_none.max(build_ore).max(build_clay).max(build_obsidian).max(build_geode)
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    blueprints: Vec<Blueprint>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut blueprints = vec![];
        for line in &raw {
            if line.is_empty() { continue }
            let words: Vec<&str> = line.split_whitespace().collect();
            let ore: i32 = words[6].parse().unwrap();
            let clay: i32 = words[12].parse().unwrap();
            let obsidian: (i32, i32) = (words[18].parse().unwrap(), words[21].parse().unwrap());
            let geode: (i32, i32) = (words[27].parse().unwrap(), words[30].parse().unwrap());
            blueprints.push(Blueprint {
                ore,
                clay,
                obsidian,
                geode
            });

        }
        Self {
            raw: raw.clone(),
            blueprints,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut sum = 0;
        // for (index, blueprint) in self.blueprints.iter().enumerate() {
        //     let geodes = blueprint.get(0, (0, 0, 0), (1, 0, 0, 0));
        //     println!("{}", geodes);
        //     sum += (index + 1) as i32 * geodes;
        // }
        sum
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
