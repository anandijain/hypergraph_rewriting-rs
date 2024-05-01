use core::hash;

/// note about the structure is that you cannot have vertex 3 without 2 and 1 at least 
/// being considered empty (not included in any hyperedge)
#[derive(Debug, Clone, Eq, PartialEq)]
struct HG {
    /// has length nv and each element answers the question,
    /// "which indices into he2v have my index?"
    /// so the hypergraph with 2 vertices and 0 edges looks like
    /// [[], []]
    v2he: Vec<Vec<usize>>,
    /// has length ne
    /// so example no edge 2 v graph is []
    /// then when we add a edge (0, 1) then we have
    /// v2he: [[0], [0]]
    /// he2v: [[0, 1]]
    he2v: Vec<Vec<usize>>,
}

impl HG {
    fn new() -> Self {
        HG {
            v2he: Vec::new(),
            he2v: Vec::new(),
        }
    }

    fn add_vertex(&mut self) {
        self.v2he.push(Vec::new());
    }

    fn add_vertices(&mut self, n: usize) {
        for _ in 0..n {
            self.add_vertex();
        }
    }

    fn add_edge(&mut self, vertices: Vec<usize>) {
        let he = self.he2v.len();
        self.he2v.push(vertices.clone());
        for v in vertices {
            self.v2he[v as usize].push(he);
        }
    }

    fn add_edges(&mut self, edges: Vec<Vec<usize>>) {
        for edge in edges {
            self.add_edge(edge);
        }
    }

    fn nv(&self) -> usize {
        self.v2he.len()
    }

    fn ne(&self) -> usize {
        self.he2v.len()
    }

    fn rem_vertex(&mut self, v: usize) {
        let mut he = self.v2he[v].clone();
        for h in he {
            self.rem_edge(h);
        }
        self.v2he[v] = Vec::new();
    }

    fn rem_edge(&mut self, h: usize) {
        let mut v = self.he2v[h].clone();
        for vv in v {
            self.v2he[vv] = self.v2he[vv]
                .iter()
                .filter(|&x| *x != h)
                .map(|x| *x)
                .collect();
        }
        self.he2v[h] = Vec::new();
    }

    // fn from_edges(edges: Vec<Vec<usize>>) -> Self {
    //     let mut hg = HG::new();
    //     // the 
    //     hg.add_edges(edges);
    //     hg
    // }

}

fn main() {
    let mut hg = HG::new();
    hg.add_vertices(2);
    println!("{:?}", hg);
    hg.add_edge(vec![0, 1]);
    println!("{:?}", hg);

    // https://en.wikipedia.org/wiki/Hypergraph#/media/File:Hypergraph-wikipedia.svg
    // the following is the graph from the above picture
    let mut g = HG::new();
    g.add_vertices(7);
    let es = vec![vec![0, 1, 2], vec![1, 2], vec![2, 4, 5], vec![3]];
    g.add_edges(es);

    println!("{:?}", g);
}
