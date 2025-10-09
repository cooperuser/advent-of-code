use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};
use utils::prelude::*;

use utils::{vector::Vector, vector3::Vector3};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    blocks: Vec<Link>,
}

type Block = (Vector3, Vector3);

#[derive(Default)]
struct Link {
    above: HashSet<usize>,
    below: HashSet<usize>,
}

fn aabb(a: Block, b: Block) -> bool {
    Vector::aabb((a.0.xy(), a.1.xy()), (b.0.xy(), b.1.xy()))
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 5,
            answer_b: 7,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let one = Vector3::new(1, 1, 1);
        let mut blocks: Vec<_> = raw
            .iter()
            .map(|line| {
                let (left, right) = line.split_once('~').unwrap();
                let left: Vec<_> = left.split(',').map(|n| n.parse().unwrap()).collect();
                let right: Vec<_> = right.split(',').map(|n| n.parse().unwrap()).collect();
                let left = Vector3::new(left[0], left[1], left[2]);
                let right = Vector3::new(right[0], right[1], right[2]);
                (left, Vector3::add(right, one))
            })
            .collect();
        blocks.sort_by_key(|block| block.0.z);

        for a in 0..blocks.len() {
            'fall: while blocks[a].0.z > 0 {
                for b in 0..a {
                    let a = blocks[a];
                    let b = blocks[b];
                    if b.1.z == a.0.z && aabb(a, b) {
                        break 'fall;
                    }
                }

                blocks[a].0.z -= 1;
                blocks[a].1.z -= 1;
            }
        }

        let mut links = Vec::new();
        for block in blocks.iter() {
            let mut link = Link::default();
            for (index, other) in blocks.iter().enumerate() {
                let intersecting = aabb(*block, *other);
                if block.1.z == other.0.z && intersecting {
                    link.above.insert(index);
                    continue;
                }
                if other.1.z == block.0.z && intersecting {
                    link.below.insert(index);
                    continue;
                }
            }
            links.push(link);
        }

        Self {
            raw: raw.clone(),
            blocks: links,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        'link: for link in self.blocks.iter() {
            for &block in &link.above {
                if self.blocks[block].below.len() == 1 {
                    continue 'link;
                }
            }
            count += 1;
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        let mut map: HashMap<usize, i64> = HashMap::new();
        for (index, block) in self.blocks.iter().enumerate() {
            let mut count = 0;
            let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
            let mut removable: HashSet<usize> = HashSet::from([index]);
            let mut seen: HashSet<usize> = HashSet::new();
            heap.extend(block.above.iter().map(|&b| Reverse(b)));

            while let Some(Reverse(next)) = heap.pop() {
                if !seen.insert(next) {
                    continue;
                }

                if removable.is_superset(&self.blocks[next].below) {
                    heap.extend(self.blocks[next].above.iter().map(|&b| Reverse(b)));
                    removable.insert(next);
                    count += 1;
                }
            }

            map.insert(index, count);
        }

        Some(map.values().sum())
    }
}

utils::solution::test_solution!(aoc2023, day22);
