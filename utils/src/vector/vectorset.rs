use std::{collections::BTreeSet, rc::Rc};

use super::Vector;

#[derive(Debug, Clone)]
pub struct VectorSet {
    grid: Vec<Vec<bool>>,
    size: Vector,
    count: usize,
}

impl VectorSet {
    #[allow(dead_code)]
    pub fn new(size: Vector) -> Self {
        Self {
            grid: vec![vec![false; size.x as usize]; size.y as usize],
            size,
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn from_grid(grid: &[Rc<str>], ch: char) -> VectorSet {
        let size = Vector::new_usize(grid[0].len(), grid.len());
        let mut set = VectorSet::new(size);

        for (y, line) in grid.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == ch {
                    set.insert(Vector::new_usize(x, y));
                }
            }
        }

        set
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector) -> Option<bool> {
        if !pos.contained_in(Vector::zero(), self.size) {
            return None;
        }

        let old = self.grid[pos.y as usize][pos.x as usize];
        self.grid[pos.y as usize][pos.x as usize] = true;
        if !old {
            self.count += 1;
        }
        Some(!old)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector) -> Option<bool> {
        if !pos.contained_in(Vector::zero(), self.size) {
            return None;
        }

        let old = self.grid[pos.y as usize][pos.x as usize];
        self.grid[pos.y as usize][pos.x as usize] = false;
        if old {
            self.count -= 1;
        }
        Some(old)
    }

    #[allow(dead_code)]
    pub fn contains(&self, pos: Vector) -> bool {
        if !pos.contained_in(Vector::zero(), self.size) {
            return false;
        }

        self.grid[pos.y as usize][pos.x as usize]
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> VectorSetIterator<'_> {
        VectorSetIterator {
            set: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.count
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[allow(dead_code)]
    pub fn extend(&mut self, other: &VectorSet) {
        for v in other.iter() {
            self.insert(v);
        }
    }

    pub fn get_orientations(&self) -> Vec<VectorSet> {
        let s = self.size - Vector::new(1, 1);
        let a = self.clone();
        let mut b = VectorSet::new(self.size);
        let mut c = VectorSet::new(self.size);
        let mut d = VectorSet::new(self.size);
        let mut e = VectorSet::new(self.size.flip());
        let mut f = VectorSet::new(self.size.flip());
        let mut g = VectorSet::new(self.size.flip());
        let mut h = VectorSet::new(self.size.flip());
        for p in self.iter() {
            b.insert(Vector::new(s.x - p.x, p.y));
            c.insert(Vector::new(p.x, s.y - p.y));
            d.insert(Vector::new(s.x - p.x, s.y - p.y));
            e.insert(Vector::new(p.y, p.x));
            f.insert(Vector::new(s.y - p.y, p.x));
            g.insert(Vector::new(p.y, s.x - p.x));
            h.insert(Vector::new(s.y - p.y, s.x - p.x));
        }

        let mut set: BTreeSet<(BTreeSet<Vector>, Vector)> = BTreeSet::new();
        set.insert((a.iter().collect(), self.size));
        set.insert((b.iter().collect(), self.size));
        set.insert((c.iter().collect(), self.size));
        set.insert((d.iter().collect(), self.size));
        set.insert((e.iter().collect(), self.size.flip()));
        set.insert((f.iter().collect(), self.size.flip()));
        set.insert((g.iter().collect(), self.size.flip()));
        set.insert((h.iter().collect(), self.size.flip()));

        set.into_iter()
            .map(|(s, size)| {
                let mut set = VectorSet::new(size);
                for pos in s {
                    set.insert(pos);
                }
                set
            })
            .collect()
    }
}

pub struct VectorSetIterator<'a> {
    set: &'a VectorSet,
    index: i64,
}

impl<'a> Iterator for VectorSetIterator<'a> {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.set.size.area() {
                return None;
            }
            let pos = Vector::new(self.index % self.set.size.x, self.index / self.set.size.x);
            self.index += 1;
            if self.set.contains(pos) {
                return Some(pos);
            }
        }
    }
}

pub struct VectorSetIntoIterator {
    set: VectorSet,
    index: i64,
}

impl Iterator for VectorSetIntoIterator {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.set.size.area() {
                return None;
            }
            let pos = Vector::new(self.index % self.set.size.x, self.index / self.set.size.x);
            self.index += 1;
            if self.set.contains(pos) {
                return Some(pos);
            }
        }
    }
}

impl IntoIterator for VectorSet {
    type Item = Vector;
    type IntoIter = VectorSetIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            set: self,
            index: 0,
        }
    }
}

impl std::fmt::Display for VectorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(
                    f,
                    "{}",
                    if self.contains(Vector::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_by_one_empty() {
        let set = VectorSet::new(Vector::new(1, 1));
        assert_eq!(set.iter().next(), None);
    }

    #[test]
    fn one_by_one_filled() {
        let mut set = VectorSet::new(Vector::new(1, 1));
        set.insert(Vector::new(0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_empty() {
        let set = VectorSet::new(Vector::new(2, 2));
        let mut iter = set.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_1() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_2() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_3() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_4() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_1() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), Some(Vector::new(1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_2() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(0, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), Some(Vector::new(0, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_3() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(1, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), Some(Vector::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_4() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0));
        set.insert(Vector::new(0, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(1, 0)));
        assert_eq!(iter.next(), Some(Vector::new(0, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_5() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0));
        set.insert(Vector::new(1, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(1, 0)));
        assert_eq!(iter.next(), Some(Vector::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_6() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 1));
        set.insert(Vector::new(1, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 1)));
        assert_eq!(iter.next(), Some(Vector::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_filled() {
        let mut set = VectorSet::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(0, 1));
        set.insert(Vector::new(1, 0));
        set.insert(Vector::new(1, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector::new(0, 0)));
        assert_eq!(iter.next(), Some(Vector::new(1, 0)));
        assert_eq!(iter.next(), Some(Vector::new(0, 1)));
        assert_eq!(iter.next(), Some(Vector::new(1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn orientations_all_8() {
        let mut set = VectorSet::new(Vector::new(3, 3));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(1, 0));
        let orientations = set.get_orientations();
        assert_eq!(orientations.len(), 8);
    }

    #[test]
    fn orientations_only_4() {
        let mut set = VectorSet::new(Vector::new(3, 3));
        set.insert(Vector::new(0, 0));
        let orientations = set.get_orientations();
        assert_eq!(orientations.len(), 4);
    }

    #[test]
    fn orientations_only_2() {
        let mut set = VectorSet::new(Vector::new(3, 3));
        set.insert(Vector::new(0, 0));
        set.insert(Vector::new(2, 2));
        let orientations = set.get_orientations();
        assert_eq!(orientations.len(), 2);
    }
}
