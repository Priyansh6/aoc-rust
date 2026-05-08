pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        // Path compression: make everything point directly to root
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }

        root
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}
