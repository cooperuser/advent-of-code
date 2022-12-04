use std::{ops::Range, collections::HashSet};

struct Day04;

impl Day04 {
    fn part_a(input: Vec<&str>) -> i32 {
        let lines: Vec<(Range<i32>, Range<i32>)> = input.iter()
            .map(|line| {
                let ranges = line.split(",")
                    .map(|side| {
                        let v = side.split("-")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        v[0]..v[1] + 1
                    }).collect::<Vec<Range<i32>>>();
                (ranges[0].clone(), ranges[1].clone())
            }).collect();

        let mut count = 0;
        for line in lines {
            let a = line.0.collect::<HashSet<i32>>();
            let b = line.1.collect::<HashSet<i32>>();
            let inter = a.intersection(&b).collect::<HashSet<&i32>>().len();
            if inter == a.len().min(b.len()) {
                count += 1;
            }
        }
        count
    }

    fn part_b(input: Vec<&str>) -> i32 {
        let lines: Vec<(Range<i32>, Range<i32>)> = input.iter()
            .map(|line| {
                let ranges = line.split(",")
                    .map(|side| {
                        let v = side.split("-")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        v[0]..v[1] + 1
                    }).collect::<Vec<Range<i32>>>();
                (ranges[0].clone(), ranges[1].clone())
            }).collect();

        let mut count = 0;
        for line in lines {
            let a = line.0.collect::<HashSet<i32>>();
            let b = line.1.collect::<HashSet<i32>>();
            let union = a.union(&b).collect::<HashSet<&i32>>().len();
            if union != a.len() + b.len() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        // let input = include_str!("input_sample.txt");
        let input = include_str!("input.txt");
        // let split = input.split("\n");
        let split = input.split("\n").filter(|line| !line.is_empty());
        assert_eq!(Day04::part_a(split.collect()), 651);
    }

    #[test]
    fn part_b() {
        // let input = include_str!("input_sample.txt");
        let input = include_str!("input.txt");
        // let split = input.split("\n");
        let split = input.split("\n").filter(|line| !line.is_empty());
        assert_eq!(Day04::part_b(split.collect()), 956);
    }
}
