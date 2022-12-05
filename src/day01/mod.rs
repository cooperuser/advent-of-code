#![allow(dead_code)]

struct Day01;

impl Day01 {
    fn part_a(input: Vec<&str>) -> i32 {
        let mut deers = Vec::new();
        let mut deer = 0;

        for i in input {
            if i == "" {
                deers.push(deer);
                deer = 0;
            } else {
                deer += i.parse::<i32>().unwrap();
            }
        }
        deers.push(deer);
        
        let mut max = 0;
        for deer in deers {
            max = max.max(deer);
        }

        max
    }

    fn part_b(input: Vec<&str>) -> i32 {
        let mut deers = Vec::new();
        let mut deer = 0;

        for i in input {
            if i == "" {
                deers.push(deer);
                deer = 0;
            } else {
                deer += i.parse::<i32>().unwrap();
            }
        }
        deers.push(deer);

        deers.sort_by(|a, b| { b.cmp(a) });
        deers[0] + deers[1] + deers[2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let input = include_str!("input_a.dat");
        let split = input.split("\n");
        assert_eq!(Day01::part_a(split.collect()), 24000);
    }

    #[test]
    fn part_b() {
        let input = include_str!("input_a.dat");
        let split = input.split("\n");
        assert_eq!(Day01::part_b(split.collect()), 45000);
    }
}
