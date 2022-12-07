use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 95437;
pub const SAMPLE_B: i32 = 24933642;

#[derive(Debug)]
enum DiskObject {
    File(i32),
    Folder(HashMap<String, DiskObject>)
}

pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    disk: HashMap<String, DiskObject>,
}

fn get_folder(disk: &mut HashMap<String, DiskObject>, mut path: Vec<String>) -> &mut HashMap<String, DiskObject> {
    if path.is_empty() { return disk }
    let name = path.remove(0);
    match disk.get_mut(&name).unwrap() {
        DiskObject::File(_) => panic!(),
        DiskObject::Folder(f) => {
            get_folder(f, path)
        },
    }
}

fn is_folder_small(folder: &HashMap<String, DiskObject>, candidates: &mut Vec<i32>, max_size: i32) -> i32 {
    let mut size = 0;
    for (_name, obj) in folder {
        match obj {
            DiskObject::File(f) => size += f,
            DiskObject::Folder(f) => size += is_folder_small(f, candidates, max_size)
        }
    }
    if size < max_size { candidates.push(size); }
    size
}

fn is_folder_big_enough(folder: &HashMap<String, DiskObject>, candidates: &mut Vec<i32>, total: i32) -> i32 {
    let mut size = 0;
    for (_name, obj) in folder {
        match obj {
            DiskObject::File(f) => size += f,
            DiskObject::Folder(f) => size += is_folder_big_enough(f, candidates, total)
        }
    }
    if total + size > 30000000 { candidates.push(size) }
    size
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut disk = HashMap::new();
        let mut path: Vec<String> = vec![];
        for line in &raw[1..] {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words[0] == "$" {
                match words[1] {
                    "cd" => {
                        if words[2] == ".." { path.pop(); }
                        else { path.push(words[2].to_string()); }
                    },
                    "ls" => (),
                    _ => ()
                }
            } else {
                let folder = get_folder(&mut disk, path.clone());
                if words[0] == "dir" {
                    folder.insert(words[1].to_string(), DiskObject::Folder(HashMap::new()));
                } else {
                    folder.insert(words[1].to_string(), DiskObject::File(words[0].parse::<i32>().unwrap()));
                }
            }
        }
        Self {
            raw: raw.clone(),
            disk,
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut sizes: Vec<i32> = vec![];
        is_folder_small(&self.disk, &mut sizes, 100000);
        sizes.iter().sum()
    }

    pub fn part_b(&self) -> i32 {
        let mut sizes: Vec<i32> = vec![];
        let total = 70000000 - is_folder_small(&self.disk, &mut vec![], 100000);
        is_folder_big_enough(&self.disk, &mut sizes, total);
        *sizes.iter().min().unwrap()
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
