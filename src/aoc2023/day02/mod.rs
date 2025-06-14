use crate::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    games: Vec<Vec<Set>>,
}

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 8,
            answer_b: 2286,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut games = Vec::new();

        for line in raw.iter() {
            let mut game = Vec::new();
            let sets: Vec<&str> = line.split_once(": ").unwrap().1.split("; ").collect();
            for set in sets {
                let mut cubes = Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                for pull in set.split(", ") {
                    let (count, color) = pull.split_once(' ').unwrap();
                    let count: usize = count.parse().unwrap();
                    match color {
                        "red" => cubes.red = count,
                        "green" => cubes.green = count,
                        "blue" => cubes.blue = count,
                        _ => (),
                    }
                }

                game.push(cubes);
            }

            games.push(game);
        }

        Self {
            raw: raw.clone(),
            games,
        }
    }

    fn part_a(&self) -> Option<usize> {
        let mut sum = 0;
        let cubes = Set {
            red: 12,
            green: 13,
            blue: 14,
        };

        'game: for (id, game) in self.games.iter().enumerate() {
            for set in game {
                if set.red > cubes.red || set.green > cubes.green || set.blue > cubes.blue {
                    continue 'game;
                }
            }

            sum += id + 1;
        }

        Some(sum)
    }

    fn part_b(&self) -> Option<usize> {
        let mut sum = 0;

        for game in &self.games {
            let mut min = Set {
                red: 0,
                green: 0,
                blue: 0,
            };

            for set in game {
                min.red = min.red.max(set.red);
                min.green = min.green.max(set.green);
                min.blue = min.blue.max(set.blue);
            }

            sum += min.red * min.green * min.blue;
        }

        Some(sum)
    }
}

crate::solution::test_solution!();
