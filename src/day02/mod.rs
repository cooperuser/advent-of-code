struct Day02;

enum Play {
    Rock,
    Paper,
    Scissors
}

fn code_to_play(code: &str) -> Play {
    match code {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => panic!()
    }
}

fn get_score(you: &Play, opp: &Play) -> i32 {
    match you {
        Play::Rock => 1 + match opp {
            Play::Rock => 3,
            Play::Paper => 0,
            Play::Scissors => 6,
        },
        Play::Paper => 2 + match opp {
            Play::Rock => 6,
            Play::Paper => 3,
            Play::Scissors => 0,
        },
        Play::Scissors => 3 + match opp {
            Play::Rock => 0,
            Play::Paper => 6,
            Play::Scissors => 3,
        }
    }
}

fn get_play(you: &str, opp: &Play) -> i32 {
    match you {
        "X" => 0 + match opp {
            Play::Rock => 3,
            Play::Paper => 1,
            Play::Scissors => 2,
        },
        "Y" => 3 + match opp {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        },
        "Z" => 6 + match opp {
            Play::Rock => 2,
            Play::Paper => 3,
            Play::Scissors => 1,
        },
        _ => panic!()
    }
}

impl Day02 {
    fn part_a(input: Vec<&str>) -> i32 {
        let pairs: Vec<(Play, Play)> = input.iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let raw = line.split(" ").collect::<Vec<&str>>();
                (code_to_play(raw[0]), code_to_play(raw[1]))
            }).collect();

        let totals = pairs.iter().map(|(a, b)| {
            get_score(b, a)
        });

        totals.sum()
    }

    fn part_b(input: Vec<&str>) -> i32 {
        let pairs: Vec<(Play, &str)> = input.iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let raw = line.split(" ").collect::<Vec<&str>>();
                (code_to_play(raw[0]), raw[1])
            }).collect();

        let totals = pairs.iter().map(|(a, b)| {
            get_play(b, a)
        });

        totals.sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = include_str!("input.dat");
        let split = input.split("\n");
        assert_eq!(Day02::part_a(split.collect()), 11063);
    }

    #[test]
    fn part_b() {
        let input = include_str!("input.dat");
        let split = input.split("\n");
        assert_eq!(Day02::part_b(split.collect()), 10349);
    }
}
