use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use std::hash::Hash;
use std::ops::{Add, AddAssign};

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
    V: Debug + Clone + Eq + Hash,
    E: Debug + Clone + Copy + Default + Ord + Add<Output = E> + AddAssign,
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
    pub fn add_edge(&mut self, a: &V, b: &V, value: E) -> bool {
        // if a == b {
        //     return false;
        // }
        self.map
            .entry(a.clone())
            .or_default()
            .insert(b.clone(), value);
        self.map
            .entry(b.clone())
            .or_default()
            .insert(a.clone(), value);
        false
    }

    pub fn get_edge(&self, a: &V, b: &V) -> Option<E> {
        self.map.get(a).unwrap().get(b).cloned()
    }

    #[allow(dead_code)]
    pub fn remove_node(&mut self, node: &V) -> bool {
        for edge in self.map.remove(node).unwrap().keys() {
            // self.map.get_mut(edge).unwrap().remove(node);
            if let Some(edges) = self.map.get_mut(edge) {
                edges.remove(node);
            }
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

    pub fn edges(&self, node: &V) -> HashMap<V, E> {
        self.map.get(node).unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn are_nodes_connected(&self, a: &V, b: &V) -> bool {
        let mut deque: VecDeque<V> = VecDeque::from([a.clone()]);
        let mut visited: HashSet<V> = HashSet::from([a.clone()]);
        while let Some(node) = deque.pop_front() {
            for to in self.map.get(&node).unwrap().keys() {
                if to == b {
                    return true;
                } else if !visited.insert(to.clone()) {
                    continue;
                }

                deque.push_back(to.clone());
            }
        }

        false
    }

    pub fn size_of_group(&self, start: &V) -> usize {
        let mut deque: VecDeque<V> = VecDeque::from([start.clone()]);
        let mut visited: HashSet<V> = HashSet::from([start.clone()]);
        while let Some(node) = deque.pop_front() {
            for to in self.map.get(&node).unwrap().keys() {
                if !visited.insert(to.clone()) {
                    continue;
                }

                deque.push_back(to.clone());
            }
        }
        visited.len()
    }

    /// Calculate which edges to cut to form two unconnected subgraphs.
    /// An implementation of the [Stoer-Wagner](https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm) algorithm.
    pub fn minimum_cut(&self) -> Option<Vec<(V, V)>> {
        let mut graph = self.clone();

        let mut best_phase = 0;
        let mut cut_value: Option<E> = None;
        let mut contractions: Vec<(V, V)> = Vec::new();

        for phase in 0..graph.map.len() - 1 {
            let (s, t, cut_weight) = graph.minimum_cut_phase();

            if cut_value.is_none() || cut_weight < cut_value.unwrap() {
                best_phase = phase;
                cut_value = Some(cut_weight);
            }

            contractions.push((s.clone(), t.clone()));

            for (node, cost) in graph.edges(&t) {
                let last = graph.get_edge(&s, &node).unwrap_or_default();
                graph.add_edge(&s, &node, last + cost);
            }

            graph.remove_node(&t);
        }

        let mut graph: HashMap<V, Vec<V>> = HashMap::new();
        for (s, t) in contractions.iter().take(best_phase) {
            graph.entry(s.clone()).or_default().push(t.clone());
            graph.entry(t.clone()).or_default().push(s.clone());
        }

        let mut visited: HashSet<V> = HashSet::new();
        let mut deque: VecDeque<V> = VecDeque::from([contractions[best_phase].1.clone()]);
        while let Some(node) = deque.pop_front() {
            if !visited.insert(node.clone()) {
                continue;
            }
            if let Some(edges) = graph.get(&node) {
                for edge in edges {
                    deque.push_back(edge.clone());
                }
            }
        }

        let mut bridges = Vec::new();
        for v in &visited {
            for (edge, _) in self.edges(v) {
                if !visited.contains(&edge) {
                    bridges.push((v.clone(), edge));
                }
            }
        }

        Some(bridges)
    }

    fn minimum_cut_phase(&self) -> (V, V, E) {
        let mut heap: BinaryHeap<Reverse<State<&V, E>>> = BinaryHeap::new();
        for node in self.map.keys() {
            heap.push(Reverse(State {
                node,
                distance: E::default(),
            }));
        }

        let mut cut_weight = E::default();
        let mut s: Option<&V> = None;
        let mut t: Option<&V> = None;

        while let Some(Reverse(State { node, distance })) = heap.pop() {
            s = t;
            t = Some(node);
            cut_weight = distance;

            for (edge, weight) in self.edges(node) {
                heap = heap
                    .iter()
                    .map(|Reverse(state)| {
                        Reverse(State {
                            node: state.node,
                            distance: state.distance
                                + if *state.node == edge {
                                    weight
                                } else {
                                    E::default()
                                },
                        })
                    })
                    .collect();
            }
        }

        (s.unwrap().clone(), t.unwrap().clone(), cut_weight)
    }

    // #[allow(dead_code)]
    // pub fn iter_nodes(&self) -> GraphNodeIterator<V, E> {
    //     GraphNodeIterator {
    //         graph: self,
    //         index: 0,
    //     }
    // }
    //
    // #[allow(dead_code)]
    // pub fn iter_edges(&self) -> GraphEdgeIterator<V, E> {
    //     GraphEdgeIterator {
    //         graph: self,
    //         index: 0,
    //     }
    // }
}

impl<V, E> Graph<V, E>
where
    V: Clone + Eq + Hash,
    E: Clone + Default + Ord + Add<Output = E>,
{
    #[allow(dead_code)]
    pub fn distance(&self, a: &V, b: &V) -> Option<E> {
        let mut heap: BinaryHeap<State<V, E>> = BinaryHeap::from([State {
            node: a.clone(),
            distance: E::default(),
        }]);
        let mut visited: HashSet<V> = HashSet::new();
        while let Some(State { node, distance }) = heap.pop() {
            for (to, edge) in self.map.get(&node).unwrap() {
                if to == b {
                    return Some(distance + edge.clone());
                } else if !visited.insert(to.clone()) {
                    continue;
                }

                heap.push(State {
                    node: to.clone(),
                    distance: distance.clone() + edge.clone(),
                });
            }
        }
        None
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

#[derive(Eq, PartialEq)]
struct State<V, E> {
    node: V,
    distance: E,
}

impl<V: Eq, E: Ord> Ord for State<V, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl<V: Eq, E: Ord> PartialOrd for State<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// pub struct GraphNodeIterator<'a, V, E>
// where
//     V: Clone + Eq + Hash,
//     E: Clone,
// {
//     graph: &'a Graph<V, E>,
//     index: usize,
// }
//
// pub struct GraphEdgeIterator<'a, V, E>
// where
//     V: Clone + Eq + Hash,
//     E: Clone,
// {
//     graph: &'a Graph<V, E>,
//     index: usize,
// }

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet as Set;

    #[test]
    fn are_nodes_connected() {
        let mut graph: Graph<i64, i64> = Graph::new();
        graph.add_node(&0);
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_node(&3);
        graph.add_edge(&0, &1, 1);
        graph.add_edge(&2, &3, 1);
        assert!(graph.are_nodes_connected(&0, &1));
        assert!(!graph.are_nodes_connected(&0, &2));
        assert!(!graph.are_nodes_connected(&0, &3));
        assert!(!graph.are_nodes_connected(&1, &2));
        assert!(!graph.are_nodes_connected(&1, &3));
        assert!(graph.are_nodes_connected(&2, &3));
    }

    #[test]
    fn minimum_cut() {
        let mut graph: Graph<&str, i64> = Graph::new();

        graph.add_node(&"a");
        graph.add_node(&"b");
        graph.add_node(&"c");
        graph.add_node(&"d");
        graph.add_node(&"e");
        graph.add_node(&"f");
        graph.add_node(&"g");
        graph.add_node(&"h");

        graph.add_edge(&"a", &"b", 2);
        graph.add_edge(&"a", &"e", 3);
        graph.add_edge(&"b", &"c", 3);
        graph.add_edge(&"b", &"e", 2);
        graph.add_edge(&"b", &"f", 2);
        graph.add_edge(&"c", &"d", 4);
        graph.add_edge(&"c", &"g", 2);
        graph.add_edge(&"d", &"g", 2);
        graph.add_edge(&"d", &"h", 2);
        graph.add_edge(&"e", &"f", 3);
        graph.add_edge(&"f", &"g", 1);
        graph.add_edge(&"g", &"h", 3);

        let min_cut = graph.minimum_cut();
        assert!(min_cut.is_some());

        let mut bridges: Set<Set<&str>> = Set::new();
        for bridge in min_cut.unwrap() {
            bridges.insert(Set::from([bridge.0, bridge.1]));
        }

        let expected = Set::from([Set::from(["b", "c"]), Set::from(["f", "g"])]);
        assert_eq!(expected, bridges);
    }
}
