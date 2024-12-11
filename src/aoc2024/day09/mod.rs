pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    filesystem: Vec<Space>,
}

#[derive(Clone, Copy)]
enum Space {
    Empty(i64),
    Filled(i64, i64),
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 1928,
            answer_b: 2858,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            filesystem: raw[0]
                .chars()
                .map(|n| n as i64 - '0' as i64)
                .enumerate()
                .map(|(index, size)| match index % 2 == 0 {
                    true => Space::Filled(size, index as i64 / 2),
                    false => Space::Empty(size),
                })
                .collect(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let filesystem: Vec<Space> = self
            .filesystem
            .iter()
            .flat_map(|space| match space {
                Space::Empty(size) => vec![Space::Empty(1); *size as usize],
                Space::Filled(size, index) => vec![Space::Filled(1, *index); *size as usize],
            })
            .collect();

        Some(Self::checksum(Self::defragment(&filesystem, true)))
    }

    fn part_b(&self) -> Option<i64> {
        Some(Self::checksum(Self::defragment(&self.filesystem, false)))
    }
}

impl Day {
    fn defragment(filesystem: &[Space], simple: bool) -> Vec<Space> {
        let mut filesystem = filesystem.to_vec();
        let mut filled = filesystem.len() - 1;
        let mut empty = 0;
        while filled > if simple { empty } else { 0 } {
            let space = filesystem[filled];
            filled -= 1;
            let Space::Filled(filled_size, _) = space else {
                continue;
            };
            filesystem[filled + 1] = Space::Empty(filled_size);

            if simple {
                while let Space::Filled(_, _) = filesystem[empty] {
                    empty += 1;
                }
            } else {
                empty = filesystem
                    .iter()
                    .position(|space| match space {
                        Space::Empty(size) => *size >= filled_size,
                        Space::Filled(_, _) => false,
                    })
                    .unwrap();
            }

            let Space::Empty(empty_size) = filesystem[empty] else {
                unreachable!();
            };
            filesystem[empty] = space;
            if filled_size != empty_size {
                filesystem.insert(empty + 1, Space::Empty(empty_size - filled_size));
                filled += 1;
            }
        }
        filesystem
    }

    fn checksum(filesystem: Vec<Space>) -> i64 {
        let mut sum = 0;
        let mut index = 0;
        for space in filesystem {
            let (size, id) = match space {
                Space::Empty(size) => (size, None),
                Space::Filled(size, id) => (size, Some(id)),
            };
            for _ in 0..size {
                sum += index * id.unwrap_or_default();
                index += 1;
            }
        }
        sum
    }
}

crate::solution::test_solution!();
