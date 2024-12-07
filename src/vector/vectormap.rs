use super::Vector;

#[derive(Debug, Clone)]
pub struct VectorMap<T> {
    grid: Vec<Vec<Option<T>>>,
    size: Vector,
}

impl<T: Clone> VectorMap<T> {
    #[allow(dead_code)]
    pub fn new(size: Vector) -> Self {
        Self {
            grid: vec![vec![None; size.x as usize]; size.y as usize],
            size,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector, value: T) -> Option<bool> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        let exists = self.grid[pos.y as usize][pos.x as usize].is_some();
        self.grid[pos.y as usize][pos.x as usize] = Some(value);
        Some(!exists)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        let value = self.grid[pos.y as usize][pos.x as usize].clone();
        self.grid[pos.y as usize][pos.x as usize] = None;
        value
    }

    #[allow(dead_code)]
    pub fn get(&self, pos: Vector) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        self.grid[pos.y as usize][pos.x as usize].clone()
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pos: Vector) -> Option<&mut T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }

        self.grid
            .get_mut(pos.y as usize)
            .unwrap()
            .get_mut(pos.x as usize)
            .unwrap()
            .as_mut()
    }

    #[allow(dead_code)]
    pub fn contains(&self, pos: Vector) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            return false;
        }

        self.grid[pos.y as usize][pos.x as usize].is_some()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> VectorMapIterator<T> {
        VectorMapIterator {
            set: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

pub struct VectorMapIterator<'a, T> {
    set: &'a VectorMap<T>,
    index: i64,
}

impl<'a, T: Clone> Iterator for VectorMapIterator<'a, T> {
    type Item = (Vector, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.set.size.area() {
                return None;
            }
            let pos = Vector::new(self.index % self.set.size.x, self.index / self.set.size.y);
            self.index += 1;
            if let Some(value) = self.set.get(pos) {
                return Some((pos, value));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_by_one_empty() {
        let set: VectorMap<()> = VectorMap::new(Vector::new(1, 1));
        assert_eq!(set.iter().next(), None);
    }

    #[test]
    fn one_by_one_filled() {
        let mut set: VectorMap<()> = VectorMap::new(Vector::new(1, 1));
        set.insert(Vector::new(0, 0), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_empty() {
        let set: VectorMap<()> = VectorMap::new(Vector::new(2, 2));
        let mut iter = set.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_1() {
        let mut set: VectorMap<()> = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_2() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_3() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_4() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_1() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0), ());
        set.insert(Vector::new(1, 0), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_2() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0), ());
        set.insert(Vector::new(0, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_3() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0), ());
        set.insert(Vector::new(1, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_4() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0), ());
        set.insert(Vector::new(0, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_5() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(1, 0), ());
        set.insert(Vector::new(1, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_6() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 1), ());
        set.insert(Vector::new(1, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_filled() {
        let mut set = VectorMap::new(Vector::new(2, 2));
        set.insert(Vector::new(0, 0), ());
        set.insert(Vector::new(0, 1), ());
        set.insert(Vector::new(1, 0), ());
        set.insert(Vector::new(1, 1), ());
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }
}
