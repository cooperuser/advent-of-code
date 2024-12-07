use super::Vector;

#[derive(Debug, Clone)]
pub struct VectorSet {
    grid: Vec<Vec<bool>>,
    size: Vector,
}

impl VectorSet {
    #[allow(dead_code)]
    pub fn new(size: Vector) -> Self {
        Self {
            grid: vec![vec![false; size.x as usize]; size.y as usize],
            size,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector) -> Option<bool> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        let old = self.grid[pos.y as usize][pos.x as usize];
        self.grid[pos.y as usize][pos.x as usize] = true;
        Some(!old)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector) -> Option<bool> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        let old = self.grid[pos.y as usize][pos.x as usize];
        self.grid[pos.y as usize][pos.x as usize] = false;
        Some(old)
    }

    #[allow(dead_code)]
    pub fn contains(&self, pos: Vector) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return false;
        }

        self.grid[pos.y as usize][pos.x as usize]
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> VectorSetIterator {
        VectorSetIterator {
            set: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.iter().count()
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
            let pos = Vector::new(self.index % self.set.size.x, self.index / self.set.size.y);
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
            let pos = Vector::new(self.index % self.set.size.x, self.index / self.set.size.y);
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
}
