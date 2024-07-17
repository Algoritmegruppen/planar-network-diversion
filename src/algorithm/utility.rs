use crate::structure::graph::edge::Edge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;
use std::collections::BTreeSet;

pub fn split_edges<W, E>(
    g: &UndirectedGraph<W, E>,
    f: Vec<E>,
) -> (UndirectedGraph<W, E>, impl Fn(&E) -> Option<E>)
where
    W: Weight,
    E: Edge<W>,
{
    // Make sure that all the banned edges are ordered, so we can check other edges quicker
    // let bans: BTreeSet<(usize,usize)> = f.into_iter().map(|(u,v)| if v < u {(v,u)} else {(u,v)} ).collect();
    let bans: BTreeSet<E> = f
        .into_iter()
        .map(|e| if e.from() < e.to() { e } else { e.reverse() })
        .collect();
    let extra = g.m() - bans.len();
    let old_n = g.n();
    let new_n = g.n() + extra;
    let mut m = g.n();
    let mut map = Vec::new();
    let mut split = UndirectedGraph::new(new_n);

    for u in g.vertices() {
        for e in g[&u].iter().filter(|&e| e.from() < e.to()) {
            if bans.contains(e) {
                split.add_edge(e.clone());
            } else {
                let (a, b) = e.subdivide(m);
                split.add_edge(a);
                split.add_edge(b);
                map.push(e.clone());
                m += 1;
            }
        }
    }

    (split, move |e| {
        if e.from() >= old_n {
            None
        } else if e.to() < old_n {
            Some(e.clone())
        } else {
            let b = &map[e.to() - old_n];
            if b.from() == e.from() {
                Some(b.clone())
            } else {
                Some(b.reverse())
            }
        }
    })
}

pub fn create_mirror_graph<W: Weight, E: Edge<W>>(
    graph: &UndirectedGraph<W, E>,
    s: usize,
    t: usize,
) -> UndirectedGraph<W, E> {
    let orig_n = graph.n();
    let new_n = orig_n * 2;
    let mut mirror = UndirectedGraph::new(new_n);
    for u in graph.vertices() {
        mirror[&u] = graph[&u].clone();
        if u != s && u != t {
            mirror[&(u + orig_n)] = graph[&u]
                .iter()
                .filter(|e| e.to() != s && e.to() != t)
                .map(|e| e.shift_by(orig_n as i64))
                .collect()
        }
    }
    mirror
}

#[cfg(test)]
mod test_utility {
    use super::*;
    use crate::structure::graph::edge::BasicEdge;

    #[test]
    fn test_mirror() {
        let g: UndirectedGraph<u64, BasicEdge<u64>> =
            std::fs::read_to_string("data/small_graphs/small1/small1.in")
                .unwrap()
                .parse()
                .unwrap();

        println!("{:?}", g);
        println!("{:?}", create_mirror_graph(&g, 0, 4));
    }

    #[test]
    fn test_split() {
        let g: UndirectedGraph<u64, BasicEdge<u64>> =
            std::fs::read_to_string("data/small_graphs/small1/small1.in")
                .unwrap()
                .parse()
                .unwrap();
        let bans = vec![BasicEdge::new(0, 3, 4), BasicEdge::new(1, 2, 2)];
        let b = bans.len();
        let (split, _) = split_edges(&g, bans);

        assert!(split.is_adjacent(0, 3));
        assert!(split.is_adjacent(1, 2));

        assert!(!split.is_adjacent(0, 1));
        assert!(!split.is_adjacent(2, 3));
        assert!(!split.is_adjacent(3, 4));
        assert!(!split.is_adjacent(2, 4));

        assert_eq!(split.n(), g.n() + g.m() - b);
    }
}
