use crate::algorithm::odd_path::shortest_odd_path;
use crate::algorithm::utility::split_edges;
use crate::structure::graph::edge::Edge;
use crate::structure::graph::planar_edge::PlanarEdge;
use crate::structure::graph::planar_graph::PlanarGraph;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::path_result::PathResult::*;
use crate::structure::weight::Weight;
use crate::utility::misc::{debug, repeat};
use queues::{IsQueue, Queue};

pub fn network_diversion<W: Weight>(
    planar: &PlanarGraph<W>,
    s: usize,
    t: usize,
    (du, dv): (usize, usize),
) -> Option<(W, Vec<PlanarEdge<W>>)> {
    if let Some(p) = bfs(planar.real(), s, t, (du, dv)) {
        let path = p.iter().map(|e| e.rotate_right()).collect();
        let diversion = planar
            .real()
            .N(du)
            .iter()
            .find(|l| l.to() == dv)
            .expect("The diversion edge doesn't exist")
            .clone();
        let (split, map) = split_edges(planar.dual(), path);
        match shortest_odd_path(&split, diversion.left(), diversion.right()) {
            Impossible => {
                debug(format!(
                    "No diversion set exist, no paths from {} to {} go through ({}, {}).",
                    s, t, du, dv
                ));
                None
            }
            Possible { cost, path } => {
                let mapped: Vec<PlanarEdge<W>> = path.iter().flat_map(|e| map(e)).collect();
                let rotated: Vec<PlanarEdge<W>> = mapped.iter().map(|e| e.rotate_right()).collect();
                debug(format!(
                    "We have to cut {} edges to divert the network, with a total cost of {}.",
                    path.len(),
                    cost
                ));
                if path.len() < 15 {
                    debug(format!("Dual diversion set: {:?}", mapped));
                    debug(format!("Real diversion set: {:?}\n", rotated));
                }

                Some((cost, rotated))
            }
        }
    } else {
        debug(format!("Could not find any s-t-path that doesn't use the diversion edge, no diversion is needed."));
        Some((0.into(), Vec::new()))
    }
}

fn bfs<W: Weight, E: Edge<W>>(
    graph: &UndirectedGraph<W, E>,
    s: usize,
    t: usize,
    (du, dv): (usize, usize),
) -> Option<Vec<E>> {
    let mut seen = repeat(graph.n(), false);
    let mut prev: Vec<Option<E>> = repeat(graph.n(), None);
    let mut q: Queue<usize> = Queue::new();
    seen[s] = true;
    q.add(s).ok()?;

    while let Ok(u) = q.remove() {
        for line in graph.N(u) {
            let v = line.to();
            if (u, v) != (du, dv) && (v, u) != (du, dv) && !seen[v] {
                seen[v] = true;
                q.add(v).ok()?;
                prev[v] = Some(line.clone());
                if v == t {
                    break;
                }
            }
        }
    }

    if seen[t] {
        let mut ret: Vec<E> = vec![prev[t].clone().unwrap()];
        let mut curr = ret[0].clone();
        while curr.from() != s {
            curr = prev[curr.from()].clone().unwrap();
            ret.push(curr.clone());
        }
        return Some(ret);
    }
    None
}

#[cfg(test)]
mod visualize_diversions {
    use crate::algorithm::network_diversion::{bfs, network_diversion};
    use crate::structure::graph::edge::Edge;
    use crate::structure::graph::planar_graph::PlanarGraph;
    use std::fs::File;
    use std::io::Write;

    fn visualize(folder: &str, file: &str) {
        let planar: PlanarGraph<f64> =
            std::fs::read_to_string([folder, "/", file, "/", file, ".in"].concat())
                .unwrap()
                .parse()
                .unwrap();
        let binding =
            std::fs::read_to_string([folder, "/", file, "/", file, ".diversion"].concat()).unwrap();
        let mut query = binding.lines().next().unwrap().split(' ');
        let s = query.next().unwrap().parse().unwrap();
        let t = query.next().unwrap().parse().unwrap();
        let d = (
            query.next().unwrap().parse().unwrap(),
            query.next().unwrap().parse().unwrap(),
        );

        let (_cost, diversion) = network_diversion(&planar, s, t, d).unwrap();
        let mut diverted =
            File::create([folder, "/", file, "/", file, ".diverted"].concat()).unwrap();
        for edge in diversion.clone() {
            diverted
                .write_all(format!("{} {}\n", edge.from(), edge.to()).as_bytes())
                .unwrap();
        }

        let mut diverted = planar.real().clone();
        diverted.delete_edges(&diversion);

        let path = bfs(&diverted, s, t, d);
        assert!(path.is_none());
    }

    #[test]
    #[ignore = "This is just for visualization purposes, so we can make nice plots for the report :)"]
    fn visualize_delaunay35() {
        visualize("data/delaunay_graphs/planar_delaunay_graphs", "delaunay35");
    }
}
