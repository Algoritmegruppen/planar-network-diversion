use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use Todo::*;

#[derive(Debug, Clone)]
pub enum Todo<W: Weight, E: Edge<W>> {
    Vertex(W, usize),
    Blossom(W, E),
}

impl<W: Weight, E: Edge<W>> PartialOrd for Todo<W, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Vertex(w1, _), Vertex(w2, _)) => w1.partial_cmp(w2),
            (Vertex(w1, _), Blossom(w2, _)) => (*w1 + *w1).partial_cmp(w2),
            (Blossom(w1, _), Vertex(w2, _)) => (*w1).partial_cmp(&(*w2 + *w2)),
            (Blossom(w1, _), Blossom(w2, _)) => w1.partial_cmp(w2),
        }
    }
}

impl<W: Weight, E: Edge<W>> Eq for Todo<W, E> {}

impl<W: Weight, E: Edge<W>> PartialEq<Self> for Todo<W, E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Vertex(w1, u), Vertex(w2, v)) => w1 == w2 && u == v,
            (Blossom(w1, e1), Blossom(w2, e2)) => w1 == w2 && (e1 == e2 || e1 == &e2.reverse()),
            _ => false,
        }
    }
}

impl<W: Weight, E: Edge<W>> Ord for Todo<W, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}
