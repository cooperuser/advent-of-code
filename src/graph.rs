use std::collections::HashMap;
use std::fmt::Debug;

use std::hash::Hash;

#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Clone + Eq + Hash,
    E: Clone,
{
    map: HashMap<V, HashMap<V, E>>,
}

impl<V, E> Graph<V, E>
where
    V: Clone + Eq + Hash,
    E: Clone,
{
    pub fn new() -> Self {
        Graph {
            map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, node: &V) -> bool {
        let exists = self.map.contains_key(node);
        self.map.entry(node.clone()).or_default();
        !exists
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, a: &V, b: &V, value: &E) -> bool {
        self.map
            .entry(a.clone())
            .or_default()
            .insert(b.clone(), value.clone());
        self.map
            .entry(b.clone())
            .or_default()
            .insert(a.clone(), value.clone());
        false
    }

    #[allow(dead_code)]
    pub fn remove_node(&mut self, node: &V) -> bool {
        for edge in self.map.remove(node).unwrap().keys() {
            self.map.get_mut(edge).unwrap().remove(node);
        }
        false
    }

    #[allow(dead_code)]
    pub fn remove_edge(&mut self, a: &V, b: &V) -> bool {
        self.map.entry(a.clone()).or_default().remove(b);
        self.map.entry(b.clone()).or_default().remove(a);
        false
    }

    #[allow(dead_code)]
    pub fn contains_edge(&self, a: &V, b: &V) -> bool {
        self.map.get(a).unwrap().contains_key(b)
    }
}

impl<V, E> Debug for Graph<V, E>
where
    V: Clone + Debug + Eq + Hash,
    E: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph {:?}", self.map)
    }
}
