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
    pub fn filled_with(size: Vector, generator: fn(Vector) -> Option<T>) -> Self {
        let mut grid = Vec::new();
        for y in 0..size.y {
            let mut row = Vec::new();
            for x in 0..size.x {
                row.push(generator(Vector::new(x, y)));
            }
            grid.push(row);
        }
        Self { grid, size }
    }

    #[allow(dead_code)]
    pub fn filled_with_value(size: Vector, value: T) -> Self {
        Self {
            grid: vec![vec![Some(value); size.x as usize]; size.y as usize],
            size,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector, value: T) -> Option<bool> {
        if !pos.contained_in(Vector::zero(), self.size) {
            return None;
        }

        let exists = self.grid[pos.y as usize][pos.x as usize].is_some();
        self.grid[pos.y as usize][pos.x as usize] = Some(value);
        Some(!exists)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector) -> Option<T> {
        if !pos.contained_in(Vector::zero(), self.size) {
            return None;
        }

        let value = self.grid[pos.y as usize][pos.x as usize].clone();
        self.grid[pos.y as usize][pos.x as usize] = None;
        value
    }

    #[allow(dead_code)]
    pub fn get(&self, pos: Vector) -> Option<T> {
        if !pos.contained_in(Vector::zero(), self.size) {
            return None;
        }

        self.grid[pos.y as usize][pos.x as usize].clone()
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pos: Vector) -> Option<&mut T> {
        if !pos.contained_in(Vector::zero(), self.size) {
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
        if !pos.contained_in(Vector::zero(), self.size) {
            return false;
        }

        self.grid[pos.y as usize][pos.x as usize].is_some()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> VectorMapIterator<T> {
        VectorMapIterator {
            map: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

pub struct VectorMapIterator<'a, T> {
    map: &'a VectorMap<T>,
    index: i64,
}

impl<'a, T: Clone> Iterator for VectorMapIterator<'a, T> {
    type Item = (Vector, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.map.size.area() {
                return None;
            }
            let pos = Vector::new(self.index % self.map.size.x, self.index / self.map.size.x);
            self.index += 1;
            if let Some(value) = self.map.get(pos) {
                return Some((pos, value));
            }
        }
    }
}

pub struct VectorMapIntoIterator<T> {
    map: VectorMap<T>,
    index: i64,
}

impl<T: Clone> Iterator for VectorMapIntoIterator<T> {
    type Item = (Vector, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.map.size.area() {
                return None;
            }
            let pos = Vector::new(self.index % self.map.size.x, self.index / self.map.size.x);
            self.index += 1;
            if let Some(value) = self.map.get(pos) {
                return Some((pos, value));
            }
        }
    }
}

impl<T: Clone> IntoIterator for VectorMap<T> {
    type Item = (Vector, T);
    type IntoIter = VectorMapIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            map: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_by_one_empty() {
        let map: VectorMap<()> = VectorMap::new(Vector::new(1, 1));
        assert_eq!(map.iter().next(), None);
    }

    #[test]
    fn one_by_one_filled() {
        let mut map: VectorMap<()> = VectorMap::new(Vector::new(1, 1));
        map.insert(Vector::new(0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_empty() {
        let map: VectorMap<()> = VectorMap::new(Vector::new(2, 2));
        let mut iter = map.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_1() {
        let mut map: VectorMap<()> = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_2() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_3() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_4() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(1, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_1() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 0), ());
        map.insert(Vector::new(1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_2() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 0), ());
        map.insert(Vector::new(0, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_3() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 0), ());
        map.insert(Vector::new(1, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_4() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(1, 0), ());
        map.insert(Vector::new(0, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_5() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(1, 0), ());
        map.insert(Vector::new(1, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_6() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 1), ());
        map.insert(Vector::new(1, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_filled() {
        let mut map = VectorMap::new(Vector::new(2, 2));
        map.insert(Vector::new(0, 0), ());
        map.insert(Vector::new(0, 1), ());
        map.insert(Vector::new(1, 0), ());
        map.insert(Vector::new(1, 1), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector::new(0, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 0), ())));
        assert_eq!(iter.next(), Some((Vector::new(0, 1), ())));
        assert_eq!(iter.next(), Some((Vector::new(1, 1), ())));
        assert_eq!(iter.next(), None);
    }
}
