use std::collections::HashMap;

pub struct DisjointSet {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl DisjointSet {
    /// Initializes a new DisjointSet with a given size.
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
            size,
        }
    }

    /// Apply path compression on a given index.
    /// Recursively move up the parent chain to set the root parent.
    pub fn find(&mut self, value: usize) -> usize {
        if self.parents[value] != value {
            self.parents[value] = self.find(self.parents[value])
        }

        self.parents[value]
    }

    /// Merge to sets within the DisjointSet.
    /// Returns the size of newly created set.
    pub fn union(&mut self, a: usize, b: usize) -> usize {
        let a = self.find(a);
        let b = self.find(b);
        if a != b {
            if self.sizes[a] > self.sizes[b] {
                self.parents[a] = b;
                self.sizes[b] += self.sizes[a];
                self.sizes[a] = 0;
                self.sizes[b]
            } else {
                self.parents[b] = a;
                self.sizes[a] += self.sizes[b];
                self.sizes[b] = 0;
                self.sizes[a]
            }
        } else {
            self.sizes[a]
        }
    }

    /// Apply `find` to each of the nodes in the DisjointSet.
    pub fn simplify(&mut self) {
        for i in 0..self.size {
            self.find(i);
        }
    }

    /// Create a map from root nodes to the size of the sets.
    /// You should call `.simplify()` first to ensure all parents are set to the root.
    pub fn map_sizes(&mut self) -> HashMap<usize, usize> {
        let mut map = HashMap::new();

        for i in 0..self.size {
            let parent = self.parents[i];
            map.insert(parent, self.sizes[parent]);
        }

        map
    }

    pub fn sizes(&self) -> Vec<usize> {
        self.sizes
            .iter()
            .filter_map(|&size| (size != 0).then_some(size))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut set = DisjointSet::new(8);
        set.union(0, 1);
        set.union(2, 3);
        set.union(4, 5);
        set.union(5, 7);
        set.union(1, 4);
        set.simplify();
        let mut map = HashMap::new();
        map.insert(0, 5);
        map.insert(2, 2);
        map.insert(6, 1);
        set.simplify();
        assert_eq!(set.map_sizes(), map);
    }
}
