use crate::structure::cost::{Cost, Cost::*};
use crate::structure::graph::edge::Edge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::{Order, Weight};
use crate::utility::misc::repeat;
use queues::{IsQueue, Queue};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn all_shortest_paths<W: Weight, E: Edge<W>>(
    graph: &UndirectedGraph<W, E>,
    s: usize,
) -> Vec<Cost<W>> {
    let mut dist = repeat(graph.n(), Infinite);
    let mut done = repeat(graph.n(), false);
    dist[s] = Finite(0.into());
    let mut pqv: BinaryHeap<(Reverse<Order<W>>, usize)> =
        BinaryHeap::from([(Reverse(Order(0.into())), s)]);
    while let Some((Reverse(Order(d)), u)) = pqv.pop() {
        if !done[u] {
            done[u] = true;
            for e in &graph[&u] {
                let v = e.to();
                let dv = d + e.weight();
                if Finite(dv) < dist[v] {
                    dist[v] = Finite(dv);
                    pqv.push((Reverse(Order(dv)), v));
                }
            }
        }
    }
    dist
}

pub fn bfs<W, E>(graph: &UndirectedGraph<W, E>, s: usize) -> Vec<Cost<u64>>
where
    W: Weight,
    E: Edge<W>,
{
    let mut dist = repeat(graph.n(), Infinite);
    let mut q: Queue<(usize, u64)> = Queue::new();
    q.add((s, 0)).unwrap();
    dist[s] = Finite(0);

    while let Ok((u, d)) = q.remove() {
        for e in &graph[&u] {
            let v = e.to();
            if dist[v].is_infinite() {
                dist[v] = Finite(d + 1);
                q.add((v, d + 1)).unwrap();
            }
        }
    }

    dist
}

#[cfg(test)]
mod create_worst_queries {
    use super::*;
    use crate::structure::graph::edge::BasicEdge;
    use crate::structure::graph::planar_edge::PlanarEdge;
    use crate::structure::graph::planar_graph::PlanarGraph;
    use std::fs::File;
    use std::io::Write;

    fn read_normal<W: Weight>(path: &str) -> UndirectedGraph<W, BasicEdge<W>> {
        println!("Attempted path:\n{}", path);
        std::fs::read_to_string(path)
            .expect("Could not find the graph")
            .parse()
            .expect("Could not parse the graph")
    }
    fn read_planar<W: Weight>(path: &str) -> UndirectedGraph<W, PlanarEdge<W>> {
        std::fs::read_to_string(path)
            .expect("Could not find the graph")
            .parse::<PlanarGraph<W>>()
            .expect("Could not parse the graph")
            .real()
            .clone()
    }
    #[ignore]
    #[test]
    fn find_worst_case_paths_and_diversions() {
        // create_worst_queries("data/planar_graphs/real_planar_graphs/CityOfOldenburg/CityOfOldenburg", true);
        // create_worst_queries("data/planar_graphs/real_planar_graphs/CityOfSanJoaquinCounty/CityOfSanJoaquinCounty", true);
        // create_worst_queries("data/planar_graphs/real_planar_graphs/CaliforniaRoadNetwork/CaliforniaRoadNetwork", true);
        // create_worst_queries("data/planar_graphs/real_planar_graphs/SanFranciscoRoadNetwork/SanFranciscoRoadNetwork", true);
        // create_worst_queries("data/planar_graphs/real_planar_graphs/RoadNetworkOfNorthAmerica/RoadNetworkOfNorthAmerica", true);
        // create_worst_queries("data/real_graphs/CaliforniaRoadNetwork/CaliforniaRoadNetwork", false);
        // create_worst_queries("data/real_graphs/CityOfOldenburg/CityOfOldenburg", false);
        // create_worst_queries("data/real_graphs/CityOfSanJoaquinCounty/CityOfSanJoaquinCounty", false);
        // create_worst_queries("data/real_graphs/COX2/COX2", false);
        // create_worst_queries("data/real_graphs/COX2-MD/COX2-MD", false);
        // create_worst_queries("data/real_graphs/fb-pages-government/fb-pages-government", false);
        // create_worst_queries("data/real_graphs/musae-github/musae-github", false);
        // create_worst_queries("data/real_graphs/NorthAmericaRoadNetwork/NorthAmericaRoadNetwork", false);
        // create_worst_queries("data/real_graphs/power-494-bus/power-494-bus", false);
        // create_worst_queries("data/real_graphs/power-1138-bus/power-1138-bus", false);
        // create_worst_queries("data/real_graphs/power-bcspwr09/power-bcspwr09", false);
        // create_worst_queries("data/real_graphs/power-bcspwr10/power-bcspwr10", false);
        // create_worst_queries("data/real_graphs/SanFranciscoRoadNetwork/SanFranciscoRoadNetwork", false);
        // create_worst_queries("data/real_graphs/soc-pokec-relationships/soc-pokec-relationships", false);
        // create_worst_queries("data/real_graphs/twitch/twitch", false);
        // create_worst_queries("data/real_graphs/web-EPA/web-EPA", false);
    }

    fn create_worst_queries(path: &str, diversions: bool) {
        let input = [path, ".in"].concat();
        let mut diversion = File::create([path, ".diversion"].concat()).unwrap();
        let mut odd_path = File::create([path, ".path"].concat()).unwrap();
        // let graph = read_planar::<f64>(input.as_str());
        let graph = read_normal::<u64>(input.as_str());
        let mut worst_s = 0;
        let mut worst_t = 0;
        let mut worst_c = 0;
        let mut worst_d = Vec::new();
        let n = graph.n();
        for s in [
            0,
            n / 5,
            (n + 1) / 5,
            (n + 2) / 5,
            (n + 3) / 5,
            (n + 4) / 5,
            n - 1,
        ] {
            let dists = bfs(&graph, s);
            let (cost, t) = Cost::sup_index(&dists).unwrap();
            if cost > worst_c {
                worst_s = s;
                worst_c = cost;
                worst_t = t;
                worst_d = dists;
            }
        }
        let seen = worst_d.clone().iter().filter(|c| c.is_finite()).count();
        println!("{}:", input);
        println!(
            "Starting from {}, we can reach {} / {} vertices in the graph",
            worst_s,
            seen,
            graph.n()
        );
        println!(
            "The worst vertex to find from s = {} is {}, with a distance of {:?}.",
            worst_s, worst_t, worst_c
        );
        if diversions {
            println!("Suggested diversions: ");
            for c in [worst_c / 3, worst_c / 2, worst_c * 2 / 3] {
                let du = worst_d.iter().position(|u| *u == Finite(c)).unwrap();
                let dv = graph[du][0].to();
                diversion
                    .write_all(format!("{} {} {} {}\n", worst_s, worst_t, du, dv).as_bytes())
                    .unwrap();
                println!("{} {} {} {}", worst_s, worst_t, du, dv);
            }
        }
        println!("Suggested odd path: ");
        odd_path
            .write_all(format!("{} {}\n", worst_s, worst_t).as_bytes())
            .unwrap();
        println!("{} {}\n", worst_s, worst_t);
    }

    #[ignore]
    #[test]
    fn create_worst_delaunay_queries() {
        for i in (1000..=100_000).step_by(1000) {
            let path = format!(
                "data/delaunay_graphs/normal_delaunay_graphs/delaunay{}/delaunay{}",
                i, i
            );
            create_worst_queries(path.as_str(), true);
        }
    }
}
