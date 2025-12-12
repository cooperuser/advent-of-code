use super::Vector3;

#[derive(Debug, Clone)]
pub struct Vector3Set {
    grid: Vec<Vec<Vec<bool>>>,
    size: Vector3,
    count: usize,
}

impl Vector3Set {
    #[allow(dead_code)]
    pub fn new(size: Vector3) -> Self {
        Self {
            grid: vec![vec![vec![false; size.x as usize]; size.y as usize]; size.z as usize],
            size,
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector3) -> Option<bool> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        let old = self.grid[pos.z as usize][pos.y as usize][pos.x as usize];
        self.grid[pos.z as usize][pos.y as usize][pos.x as usize] = true;
        if !old {
            self.count += 1;
        }
        Some(!old)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector3) -> Option<bool> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        let old = self.grid[pos.z as usize][pos.y as usize][pos.x as usize];
        self.grid[pos.z as usize][pos.y as usize][pos.x as usize] = false;
        if old {
            self.count -= 1;
        }
        Some(old)
    }

    #[allow(dead_code)]
    pub fn contains(&self, pos: Vector3) -> bool {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return false;
        }

        self.grid[pos.z as usize][pos.y as usize][pos.x as usize]
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Vector3SetIterator<'_> {
        Vector3SetIterator {
            set: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct Vector3SetIterator<'a> {
    set: &'a Vector3Set,
    index: i64,
}

impl<'a> Iterator for Vector3SetIterator<'a> {
    type Item = Vector3;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.set.size.volume() {
                return None;
            }
            let index = self.index % (self.set.size.x * self.set.size.y);
            let pos = Vector3::new(
                index % self.set.size.x,
                index / self.set.size.x,
                self.index / (self.set.size.x * self.set.size.y),
            );
            self.index += 1;
            if self.set.contains(pos) {
                return Some(pos);
            }
        }
    }
}

pub struct Vector3SetIntoIterator {
    set: Vector3Set,
    index: i64,
}

impl Iterator for Vector3SetIntoIterator {
    type Item = Vector3;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.set.size.volume() {
                return None;
            }
            let index = self.index % (self.set.size.x * self.set.size.y);
            let pos = Vector3::new(
                index % self.set.size.x,
                index / self.set.size.x,
                self.index / (self.set.size.x * self.set.size.y),
            );
            self.index += 1;
            if self.set.contains(pos) {
                return Some(pos);
            }
        }
    }
}

impl IntoIterator for Vector3Set {
    type Item = Vector3;
    type IntoIter = Vector3SetIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            set: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_by_one_empty() {
        let set = Vector3Set::new(Vector3::new(1, 1, 1));
        assert_eq!(set.iter().next(), None);
    }

    #[test]
    fn one_by_one_filled() {
        let mut set = Vector3Set::new(Vector3::new(1, 1, 1));
        set.insert(Vector3::new(0, 0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_empty() {
        let set = Vector3Set::new(Vector3::new(2, 2, 1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_1() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_2() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(1, 0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(1, 0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_3() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_4() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(1, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(1, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_1() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 0, 0));
        set.insert(Vector3::new(1, 0, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_2() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 0, 0));
        set.insert(Vector3::new(0, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(0, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_3() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 0, 0));
        set.insert(Vector3::new(1, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_4() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(1, 0, 0));
        set.insert(Vector3::new(0, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(1, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(0, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_5() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(1, 0, 0));
        set.insert(Vector3::new(1, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(1, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_6() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 1, 0));
        set.insert(Vector3::new(1, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 1, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 1, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_filled() {
        let mut set = Vector3Set::new(Vector3::new(2, 2, 1));
        set.insert(Vector3::new(0, 0, 0));
        set.insert(Vector3::new(0, 1, 0));
        set.insert(Vector3::new(1, 0, 0));
        set.insert(Vector3::new(1, 1, 0));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Vector3::new(0, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 0, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(0, 1, 0)));
        assert_eq!(iter.next(), Some(Vector3::new(1, 1, 0)));
        assert_eq!(iter.next(), None);
    }
}
