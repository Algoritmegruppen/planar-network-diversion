use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;

pub enum PathResult<W: Weight, E: Edge<W>> {
    Possible { cost: W, path: Vec<E> },
    Impossible,
}
