use std::collections::HashSet;

struct Day03;

struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

fn get_priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        c as i32 - 97 + 1
    } else if c >= 'A' && c <= 'Z' {
        c as i32 - 65 + 1 + 26
    } else {
        panic!();
    }
}

impl Day03 {
    fn part_a(input: Vec<&str>) -> i32 {
        let mut rucksacks: Vec<Rucksack> = vec![];
        
        for line in input {
            let size = line.len() / 2;
            let rucksack = Rucksack {
                left: line[0..size].chars().collect(),
                right: line[size..].chars().collect(),
            };
            rucksacks.push(rucksack);
        }

        let mut priorities = 0;
        for ruck in rucksacks {
            let inter = ruck.left.intersection(&ruck.right).collect::<Vec<&char>>();
            for common in inter { priorities += get_priority(*common); }
        }

        priorities
    }

    fn part_b(input: Vec<&str>) -> i32 {
        let mut groups: Vec<(HashSet<char>, HashSet<char>, HashSet<char>)> = vec![];
        for i in 0..input.len() / 3 {
            groups.push((
                input[i * 3 + 0].chars().collect::<HashSet<char>>(),
                input[i * 3 + 1].chars().collect::<HashSet<char>>(),
                input[i * 3 + 2].chars().collect::<HashSet<char>>(),
            ));
        }

        let badges = groups.iter().map(|(a, b, c)| {
            for letter in a {
                if b.contains(letter) && c.contains(letter) {
                    return letter;
                }
            }
            panic!();
        });

        badges.map(|l| get_priority(*l)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        // let input = include_str!("input_sample.dat");
        let input = include_str!("input.dat");
        let split = input.split("\n").filter(|line| !line.is_empty());
        assert_eq!(Day03::part_a(split.collect()), 7824);
    }

    #[test]
    fn part_b() {
        // let input = include_str!("input_sample.dat");
        let input = include_str!("input.dat");
        let split = input.split("\n");
        assert_eq!(Day03::part_b(split.collect()), 2798);
    }
}
