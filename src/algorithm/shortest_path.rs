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
