use crate::structure::graph::planar_edge::PrePlanarEdge;
use crate::structure::weight::Weight;

pub trait SimpleGraphStrategy {
    fn combine<W: Weight>(a: PrePlanarEdge<W>, b: PrePlanarEdge<W>) -> PrePlanarEdge<W>;
}

pub struct KeepFirst;
impl SimpleGraphStrategy for KeepFirst {
    fn combine<W: Weight>(a: PrePlanarEdge<W>, _: PrePlanarEdge<W>) -> PrePlanarEdge<W> {
        a
    }
}

pub struct KeepHighestWeight;
impl SimpleGraphStrategy for KeepHighestWeight {
    fn combine<W: Weight>(a: PrePlanarEdge<W>, b: PrePlanarEdge<W>) -> PrePlanarEdge<W> {
        if a.weight > b.weight {
            a
        } else {
            b
        }
    }
}

pub struct KeepLowestWeight;
impl SimpleGraphStrategy for KeepLowestWeight {
    fn combine<W: Weight>(a: PrePlanarEdge<W>, b: PrePlanarEdge<W>) -> PrePlanarEdge<W> {
        if a.weight < b.weight {
            a
        } else {
            b
        }
    }
}

pub struct SumWeights;
impl SimpleGraphStrategy for SumWeights {
    fn combine<W: Weight>(a: PrePlanarEdge<W>, b: PrePlanarEdge<W>) -> PrePlanarEdge<W> {
        PrePlanarEdge {
            from: a.from,
            to: a.to,
            weight: a.weight + b.weight,
            left: a.left.or(b.left),
            right: a.right.or(b.right),
        }
    }
}
