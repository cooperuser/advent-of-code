#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 8;
pub const SAMPLE_B: i64 = 2286;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    games: Vec<Game>,
}

#[derive(Debug)]
struct Picking {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: usize,
    pickings: Vec<Picking>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut games = Vec::new();
        for (id, line) in raw.iter().enumerate() {
            let mut pickings = Vec::new();
            let groups = line
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(";")
                .map(|g| g.split(",").map(|c| c.trim().split_whitespace()));
            for group in groups {
                let mut picking = Picking { red: 0, green: 0, blue: 0 };
                for mut cube in group {
                    let count = cube.nth(0).unwrap().parse().unwrap();
                    let color = cube.last().unwrap();
                    match color {
                        "red" => picking.red = count,
                        "green" => picking.green = count,
                        "blue" => picking.blue = count,
                        _ => ()
                    }
                }
                pickings.push(picking);
            }
            games.push(Game { id: id + 1, pickings });
        }
        Self {
            raw: raw.clone(),
            games,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let max = Picking { red: 12, green: 13, blue: 14 };
        let mut sum = 0;
        'game: for game in &self.games {
            for p in &game.pickings {
                if p.red > max.red || p.green > max.green || p.blue > max.blue {
                    continue 'game;
                }
            }
            sum += game.id;
        }
        Some(sum as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for game in &self.games {
            let min = Picking {
                red: game.pickings.iter().map(|p| p.red).max().unwrap(),
                green: game.pickings.iter().map(|p| p.green).max().unwrap(),
                blue: game.pickings.iter().map(|p| p.blue).max().unwrap(),
            };
            sum += min.red * min.green * min.blue;
        }
        Some(sum as i64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a().unwrap_or(0), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b().unwrap_or(0), SAMPLE_B);
    }
}
