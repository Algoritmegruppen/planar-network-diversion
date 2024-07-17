use crate::algorithm::utility;
use crate::structure::basis::{Basis, UnionFindBase};
use crate::structure::cost::{Cost, Cost::*};
use crate::structure::graph::edge::Edge;
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::path_result::{PathResult, PathResult::*};
use crate::structure::todo::{Todo, Todo::*};
use crate::structure::weight::Weight;
use crate::utility::misc::{debug, repeat};
use std::cmp::Reverse;
use std::collections::BinaryHeap;


pub struct DerigsAlgorithm<W: Weight, E: Edge<W>> {
    graph: UndirectedGraph<W, E>,
    d_plus: Vec<Cost<W>>,
    d_minus: Vec<Cost<W>>,
    pred: Vec<Option<E>>,
    basis: UnionFindBase,
    s: usize,
    t: usize,
    orig_n: usize,
    completed: Vec<bool>,
    pq: BinaryHeap<Reverse<Todo<W, E>>>,
    in_current_blossom: Vec<bool>,
}

fn is_unique<W: Weight, E: Edge<W>>(vec: &Vec<E>) -> bool {
    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            if vec[i].to() == vec[j].to() {
                return false;
            }
        }
    }
    true
}

fn is_walk<W: Weight, E: Edge<W>>(_graph: &UndirectedGraph<W, E>, path: &Vec<E>) -> bool {
    for i in 1..path.len() {
        if path[i - 1].to() != path[i].from() {
            println!(
                "Not same endpoint: ({}, {}))",
                path[i - 1].to(),
                path[i].from()
            );
            return false;
        }
    }
    true
}

/**
Problem: Shortest Odd Path
In: an undirected graph G, two vertices s,t in V(G)
Out: the shortest s-t-path in G that uses an odd number of edges, if one exists.
*/

pub fn shortest_odd_path<W: Weight, E: Edge<W>>(
    graph: &UndirectedGraph<W, E>,
    s: usize,
    t: usize,
) -> PathResult<W, E> {
    DerigsAlgorithm::init(graph, s, t).solve()
}

fn is_simple<W: Weight, E: Edge<W>>(graph: &UndirectedGraph<W, E>, path: &Vec<E>) -> bool {
    return is_unique(&path) && is_walk(&graph, &path);
}

impl<W: Weight, E: Edge<W>> DerigsAlgorithm<W, E> {
    fn init(graph: &UndirectedGraph<W, E>, s: usize, t: usize) -> Self
    where
        Self: Sized,
    {
        let mirror_graph = utility::create_mirror_graph(graph, s, t);
        let n = mirror_graph.n();

        debug(format!(
            "\n\nLooking for an odd {}-{}-path here:\n{:?}\n",
            s, t, mirror_graph
        ));

        let mut d_plus = repeat(n, Infinite);
        let mut d_minus = repeat(n, Infinite);
        let mut pred = repeat(n, None);
        let mut completed = repeat(n, false);
        let mut pq = BinaryHeap::new();
        d_plus[s] = Finite(0.into());

        for e in &mirror_graph[&s] {
            pq.push(Reverse(Vertex(e.weight(), e.to())));
            d_minus[e.to()] = Finite(e.weight());
            pred[e.to()] = Some(e.clone());
        }
        completed[s] = true;
        completed[s + graph.n()] = true;

        DerigsAlgorithm {
            graph: mirror_graph,
            d_plus,
            d_minus,
            pred,
            basis: UnionFindBase::new(n),
            s,
            t,
            orig_n: graph.n(),
            completed,
            pq,
            in_current_blossom: repeat(n, false),
        }
    }

    fn solve(&mut self) -> PathResult<W, E> {
        if self.s == self.t {
            return Impossible;
        }

        while !self.control() {}

        if self.d_minus[self.t].is_infinite() {
            debug(format!(
                "\n\nWe can now definitely conclude that no odd {}-{}-path exists.\n\n",
                self.s, self.t
            ));
            return Impossible;
        }

        let mut curr = self.pred[self.t].clone().unwrap();
        let mut cost = curr.weight();
        let mut path = vec![curr.clone()];
        while curr.from() != self.s {
            curr = self.pred[self.mirror(curr.from())].clone().unwrap();
            cost = cost + curr.weight();
            if curr.from() < self.orig_n {
                path.push(curr.clone());
            } else {
                path.push(curr.shift_by(-(self.orig_n as i64)))
            }
        }
        path.reverse();
        debug(format!("Path of cost {} is possible: {:?\n\n}", cost, path));
        assert!(is_simple(&self.graph, &path));
        Possible { cost, path }
    }
    // Return true if the search is done. Either because we found the shortest odd s-t-path, or because none exist.
    fn control(&mut self) -> bool {
        while let Some(Reverse(todo)) = self.pq.peek() {
            match todo {
                Vertex(_, u) => {
                    if self.completed[*u] {
                        self.pq.pop();
                    } else {
                        break;
                    }
                }
                Blossom(_, e) => {
                    if self.basis.same_base(e.from(), e.to()) {
                        self.pq.pop();
                    } else {
                        break;
                    }
                }
            }
        }

        match self.pq.pop() {
            None => return true, // No odd path exists :(
            Some(Reverse(Vertex(_, u))) => {
                if u == self.t {
                    return true;
                } // Shortest odd path has been found :)
                let m = self.mirror(u);
                self.d_plus[m] = self.d_minus[u];
                self.scan(m);
            }
            Some(Reverse(Blossom(_, e))) => {
                self.blossom(&e);
            }
        }

        return false;
    }

    fn scan(&mut self, u: usize) {
        self.completed[u] = true;
        let dist_u = self.d_plus[u].expect(
            format!(
                "        We called self.scan({}), but self.d_plus[{}] is undefined!",
                u, u
            )
            .as_str(),
        );
        for e in &self.graph[&u] {
            let w = e.weight();
            let v = e.to();
            let new_dist_v = dist_u + w;
            if !self.completed[v] {
                if Finite(new_dist_v) >= self.d_minus[v] {
                    continue;
                }

                self.d_minus[v] = Finite(new_dist_v);
                self.pred[v] = Some(e.clone());
                self.pq.push(Reverse(Vertex(new_dist_v, v)));
            } else if let (Finite(dist_v), true) = (self.d_plus[v], !self.basis.same_base(u, v)) {
                self.pq
                    .push(Reverse(Blossom(dist_u + dist_v + w, e.clone())));
                if Finite(new_dist_v) < self.d_minus[v] {
                    self.d_minus[v] = Finite(new_dist_v);
                    self.pred[v] = Some(e.clone());
                }
            }
        }
    }

    fn blossom(&mut self, e: &E) {
        let (b, p1, p2) = self.backtrack_blossom(e);

        let s1 = self.set_blossom_values(&p1);
        let s2 = self.set_blossom_values(&p2);

        self.set_edge_bases(b, &p1);
        self.set_edge_bases(b, &p2);

        for u in s1 {
            self.scan(u);
        }
        for v in s2 {
            self.scan(v);
        }
    }

    fn backtrack_blossom(&mut self, e: &E) -> (usize, Vec<E>, Vec<E>) {
        let mut p1: Vec<E> = vec![e.reverse()];
        let mut p2: Vec<E> = vec![e.clone()];

        let mut u = self.basis[e.to()];
        let mut v = self.basis[e.from()];

        self.in_current_blossom[u] = true;
        self.in_current_blossom[v] = true;

        loop {
            if u != self.s {
                u = self.basis[self.mirror(u)];
                self.in_current_blossom[u] = true;
                let e = self.pred[u].clone().expect(
                    format!("    Tried to unwrap pred[{}], but it's not defined!", u).as_str(),
                );
                u = self.basis[e.from()];
                p1.push(e);

                if self.in_current_blossom[u] {
                    p1.pop();
                    self.in_current_blossom[u] = false;
                    while let Some(e) = p2.last() {
                        let vv = self.basis[e.from()];
                        self.in_current_blossom[vv] = false;
                        p2.pop();
                        if vv == u {
                            break;
                        }
                    }
                    return (u, p1, p2);
                }
                self.in_current_blossom[u] = true;
            }
            if v != self.s {
                v = self.basis[self.mirror(v)];
                self.in_current_blossom[v] = true;
                let e = self.pred[v].clone().expect(
                    format!("    Tried to unwrap pred[{}], but it's not defined!", v).as_str(),
                );
                v = self.basis[e.from()];
                p2.push(e);

                if self.in_current_blossom[v] {
                    p2.pop();
                    self.in_current_blossom[v] = false;
                    while let Some(e) = p1.last() {
                        let uu = self.basis[e.from()];
                        self.in_current_blossom[uu] = false;
                        p1.pop();
                        if uu == v {
                            break;
                        }
                    }
                    return (v, p1, p2);
                }
                self.in_current_blossom[v] = true;
            }
        }
    }

    fn set_edge_bases(&mut self, b: usize, path: &Vec<E>) {
        for e in path {
            let u = e.from();
            let m = self.mirror(u);
            self.basis.set_base(u, b);
            self.basis.set_base(m, b);
        }
    }

    fn set_blossom_values(&mut self, path: &Vec<E>) -> Vec<usize> {
        let mut ret = Vec::new();
        for e in path {
            let u = e.from();
            let v = e.to();
            let w = e.weight();
            self.in_current_blossom[u] = false;
            self.in_current_blossom[v] = false;

            // We can set a d_minus
            if self.d_plus[v] + Finite(w) < self.d_minus[u] {
                self.d_minus[u] = self.d_plus[v] + Finite(w);
                self.pred[u] = Some(e.reverse());
            }

            let m = self.mirror(u);
            // We can set a d_plus, and scan it
            if self.d_minus[u] < self.d_plus[m] {
                self.d_plus[m] = self.d_minus[u];
                ret.push(m);
            }
        }

        ret
    }

    fn mirror(&self, u: usize) -> usize {
        if u < self.orig_n {
            u + self.orig_n
        } else {
            u - self.orig_n
        }
    }
}
