use hypergraph_rewriting::hypergraph::*;

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
