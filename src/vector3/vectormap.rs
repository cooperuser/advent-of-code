use super::Vector3;

#[derive(Debug, Clone)]
pub struct Vector3Map<T> {
    grid: Vec<Vec<Vec<Option<T>>>>,
    size: Vector3,
}

impl<T: Clone> Vector3Map<T> {
    #[allow(dead_code)]
    pub fn new(size: Vector3) -> Self {
        Self {
            grid: vec![vec![vec![None; size.x as usize]; size.y as usize]; size.z as usize],
            size,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, pos: Vector3, value: T) -> Option<bool> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        let exists = self.grid[pos.z as usize][pos.y as usize][pos.x as usize].is_some();
        self.grid[pos.z as usize][pos.y as usize][pos.x as usize] = Some(value);
        Some(!exists)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, pos: Vector3) -> Option<T> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        let value = self.grid[pos.z as usize][pos.y as usize][pos.x as usize].clone();
        self.grid[pos.z as usize][pos.y as usize][pos.x as usize] = None;
        value
    }

    #[allow(dead_code)]
    pub fn get(&self, pos: Vector3) -> Option<T> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        self.grid[pos.z as usize][pos.y as usize][pos.x as usize].clone()
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, pos: Vector3) -> Option<&mut T> {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return None;
        }

        self.grid
            .get_mut(pos.z as usize)
            .unwrap()
            .get_mut(pos.y as usize)
            .unwrap()
            .get_mut(pos.x as usize)
            .unwrap()
            .as_mut()
    }

    #[allow(dead_code)]
    pub fn contains(&self, pos: Vector3) -> bool {
        if !pos.contained_in(Vector3::zero(), self.size) {
            return false;
        }

        self.grid[pos.z as usize][pos.y as usize][pos.x as usize].is_some()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Vector3MapIterator<T> {
        Vector3MapIterator {
            map: self,
            index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

pub struct Vector3MapIterator<'a, T> {
    map: &'a Vector3Map<T>,
    index: i64,
}

impl<'a, T: Clone> Iterator for Vector3MapIterator<'a, T> {
    type Item = (Vector3, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.map.size.volume() {
                return None;
            }
            let index = self.index % (self.map.size.x * self.map.size.y);
            let pos = Vector3::new(
                index % self.map.size.x,
                index / self.map.size.x,
                self.index / (self.map.size.x * self.map.size.y),
            );
            self.index += 1;
            if let Some(value) = self.map.get(pos) {
                return Some((pos, value));
            }
        }
    }
}

pub struct Vector3MapIntoIterator<T> {
    map: Vector3Map<T>,
    index: i64,
}

impl<T: Clone> Iterator for Vector3MapIntoIterator<T> {
    type Item = (Vector3, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index > self.map.size.volume() {
                return None;
            }
            let index = self.index % (self.map.size.x * self.map.size.y);
            let pos = Vector3::new(
                index % self.map.size.x,
                index / self.map.size.x,
                self.index / (self.map.size.x * self.map.size.y),
            );
            self.index += 1;
            if let Some(value) = self.map.get(pos) {
                return Some((pos, value));
            }
        }
    }
}

impl<T: Clone> IntoIterator for Vector3Map<T> {
    type Item = (Vector3, T);
    type IntoIter = Vector3MapIntoIterator<T>;

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
        let map: Vector3Map<()> = Vector3Map::new(Vector3::new(1, 1, 1));
        assert_eq!(map.iter().next(), None);
    }

    #[test]
    fn one_by_one_filled() {
        let mut map: Vector3Map<()> = Vector3Map::new(Vector3::new(1, 1, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_empty() {
        let map: Vector3Map<()> = Vector3Map::new(Vector3::new(2, 2, 1));
        let mut iter = map.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_1() {
        let mut map: Vector3Map<()> = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_2() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(1, 0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(1, 0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_3() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_one_4() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(1, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(1, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_1() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        map.insert(Vector3::new(1, 0, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 0, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_2() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        map.insert(Vector3::new(0, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(0, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_3() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        map.insert(Vector3::new(1, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_4() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(1, 0, 0), ());
        map.insert(Vector3::new(0, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(1, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(0, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_5() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(1, 0, 0), ());
        map.insert(Vector3::new(1, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(1, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_with_two_6() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 1, 0), ());
        map.insert(Vector3::new(1, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 1, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_by_two_filled() {
        let mut map = Vector3Map::new(Vector3::new(2, 2, 1));
        map.insert(Vector3::new(0, 0, 0), ());
        map.insert(Vector3::new(0, 1, 0), ());
        map.insert(Vector3::new(1, 0, 0), ());
        map.insert(Vector3::new(1, 1, 0), ());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Vector3::new(0, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 0, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(0, 1, 0), ())));
        assert_eq!(iter.next(), Some((Vector3::new(1, 1, 0), ())));
        assert_eq!(iter.next(), None);
    }
}
