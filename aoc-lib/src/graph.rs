use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Directed(());
pub struct Undirected(());

pub struct Graph<T, D> {
    nodes: Vec<T>,
    edges: Vec<Vec<(usize, u64)>>,
    _direction: PhantomData<D>,
}

impl<T> Graph<T, Directed> {
    #[must_use]
    pub const fn new_directed() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            _direction: PhantomData,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: u64) {
        self.edges[from].push((to, weight));
    }

    fn num_paths_dag_helper(&self, from: usize, to: usize, cache: &mut Vec<Option<u64>>) -> u64 {
        if from == to {
            return 1;
        }
        if let Some(cached) = cache[from] {
            return cached;
        }
        let count: u64 = self
            .neighbours(from)
            .iter()
            .map(|&(n, _)| self.num_paths_dag_helper(n, to, cache))
            .sum();
        cache[from] = Some(count);
        count
    }

    #[must_use]
    pub fn num_paths_dag(&self, from: usize, to: usize) -> u64 {
        self.num_paths_dag_helper(from, to, &mut vec![None; self.nodes.len()])
    }

    fn num_paths_through_dag_helper(
        &self,
        from: usize,
        to: usize,
        visited_mask: u32,
        must_visit: &[usize],
        cache: &mut HashMap<(usize, u32), u64>,
    ) -> u64 {
        let new_mask = must_visit
            .iter()
            .position(|&n| n == from)
            .map_or(visited_mask, |i| visited_mask | (1 << i));

        if from == to {
            let all_visited = (1u32 << must_visit.len()) - 1;
            return u64::from(new_mask == all_visited);
        }

        if let Some(&cached) = cache.get(&(from, new_mask)) {
            return cached;
        }

        let count: u64 = self
            .neighbours(from)
            .iter()
            .map(|&(n, _)| self.num_paths_through_dag_helper(n, to, new_mask, must_visit, cache))
            .sum();

        cache.insert((from, new_mask), count);
        count
    }

    #[must_use]
    pub fn num_paths_through_dag(&self, from: usize, to: usize, must_visit: &[usize]) -> u64 {
        self.num_paths_through_dag_helper(from, to, 0, must_visit, &mut HashMap::new())
    }
}

impl<T> Graph<T, Undirected> {
    #[must_use]
    pub const fn new_undirected() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            _direction: PhantomData,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: u64) {
        self.edges[from].push((to, weight));
        self.edges[to].push((from, weight));
    }
}

impl<T, D> Graph<T, D> {
    pub fn add_node(&mut self, val: T) -> usize {
        self.nodes.push(val);
        self.edges.push(Vec::new());
        self.nodes.len() - 1
    }

    #[must_use]
    pub fn neighbours(&self, node: usize) -> &[(usize, u64)] {
        &self.edges[node]
    }
}
