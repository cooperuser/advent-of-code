use std::collections::{HashMap, HashSet, VecDeque};

use utils::{prelude::*, vector3::Vector3};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    boxes: Vec<Vector3>,
    distances: HashMap<(usize, usize), i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 40,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut boxes = Vec::new();
        let mut min = Vector3::new(i64::MAX, i64::MAX, i64::MAX);
        let mut max = Vector3::new(i64::MIN, i64::MIN, i64::MIN);
        for line in &raw {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            min.x = min.x.min(x);
            min.y = min.y.min(y);
            min.z = min.z.min(z);
            max.x = max.x.max(x);
            max.y = max.y.max(y);
            max.z = max.z.max(z);
            boxes.push(Vector3::new(x, y, z));
        }

        let mut distances: HashMap<(usize, usize), i64> = HashMap::new();
        for (i, &a) in boxes[..boxes.len() - 1].iter().enumerate() {
            for (j, &b) in boxes[i + 1..].iter().enumerate() {
                let distance = (b - a).sqr_distance();
                distances.insert((i, i + j + 1), distance);
                // distance_map.insert((b, a), distance);
            }
        }

        Self {
            raw,
            boxes,
            distances,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let connections = if self.boxes.len() < 30 { 10 } else { 1000 };

        let mut distances: Vec<_> = self.distances.iter().collect();
        distances.sort_by_key(|a| a.1);

        let mut graph: Vec<Vec<bool>> = vec![vec![false; self.boxes.len()]; self.boxes.len()];
        for (i, _) in self.boxes.iter().enumerate() {
            graph[i][i] = true;
        }

        let mut circuits = 0;
        for (a, b) in distances.iter().map(|&(&pair, _)| pair) {
            let va = self.boxes[a];
            let vb = self.boxes[b];
            if graph[a][b] {
                println!("Skipping {:?} and {:?}", va, vb);
                continue;
            }

            println!("Connecting {:?} and {:?}", va, vb);
            graph[a][b] = true;
            graph[b][a] = true;

            #[allow(clippy::needless_range_loop)]
            for c in 0..self.boxes.len() {
                if graph[a][c] {
                    // if !graph[b][c] {
                    //     println!("{b} and {c}");
                    // }
                    graph[b][c] = true;
                    graph[c][b] = true;
                }
                if graph[b][c] && !graph[a][c] {
                    // if !graph[a][c] {
                    //     println!("{a} and {c}");
                    // }
                    graph[a][c] = true;
                    graph[c][a] = true;
                }
            }

            circuits += 1;
            if circuits >= connections {
                break;
            }
        }

        // let mut sets = Vec::new();
        // let mut visited = HashSet::new();
        // for b in 0..self.boxes.len() {
        //     if !visited.insert(b) {
        //         continue;
        //     }
        //     let mut set = HashSet::new();
        //     let mut queue = VecDeque::from([b]);
        //     while let Some(r#box) = queue.pop_front() {
        //         visited.insert(r#box);
        //         // if !visited.insert(r#box) {
        //         //     continue;
        //         // }
        //         if !set.insert(r#box) {
        //             continue;
        //         }
        //
        //         #[allow(clippy::needless_range_loop)]
        //         for c in 0..self.boxes.len() {
        //             if graph[r#box][c] {
        //                 queue.push_back(c);
        //             }
        //         }
        //     }
        //     println!("{:?}", set);
        //     sets.push(set.len() as i64);
        // }

        let mut map: HashMap<Vec<bool>, i64> = HashMap::new();
        for line in &graph {
            *map.entry(line.clone()).or_default() += 1;
        }
        println!("{:?}", map.values());

        print_graph(&graph);
        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        values.reverse();

        Some(values.iter().take(3).product())
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

fn print_graph(graph: &Vec<Vec<bool>>) {
    for y in graph {
        let mut s = Vec::new();
        for x in y {
            if *x {
                s.push("x");
            } else {
                s.push(".");
            }
        }
        println!("{}", s.join(" "));
    }
}

utils::solution::test_solution!(aoc2025, day08);
