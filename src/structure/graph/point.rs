use crate::structure::graph::edge::Edge;
use crate::structure::graph::planar_edge::PrePlanarEdge;
use crate::structure::weight::Weight;
use num::Complex;
use std::cmp::Ordering::{self, Equal};
use std::ops::{Add, Neg, Sub};

#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
    fn angle(&self) -> f64 {
        Complex::new(self.x, self.y).to_polar().1.neg()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

pub fn compare_edges_clockwise<'a, W: Weight>(
    center: &'a Point,
    points: &'a Vec<Point>,
) -> impl FnMut(&PrePlanarEdge<W>, &PrePlanarEdge<W>) -> Ordering + 'a {
    |a, b| {
        let fa = (points[a.to()] - *center).angle();
        let fb = (points[b.to()] - *center).angle();
        fa.partial_cmp(&fb).unwrap_or_else(|| Equal)
    }
}

#[cfg(test)]
mod test_points {
    use super::*;
    use crate::structure::graph::edge::map_to;
    fn new_edge(u: usize, v: usize) -> PrePlanarEdge<u64> {
        PrePlanarEdge {
            from: u,
            to: v,
            weight: 0,
            left: None,
            right: None,
        }
    }
    #[test]
    fn test_sorting() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, -1.0),
            Point::new(2.0, 7.0),
            Point::new(12.0, 6.0),
            Point::new(5.0, 3.0),
            Point::new(7.0, 10.0),
            Point::new(10.0, 3.0),
            Point::new(0.0, 4.0),
            Point::new(4.0, -2.0),
        ];
        let mut edges = vec![
            new_edge(4, 3),
            new_edge(4, 1),
            new_edge(4, 5),
            new_edge(4, 2),
            new_edge(4, 0),
            new_edge(4, 8),
            new_edge(4, 6),
            new_edge(4, 7),
        ];
        edges.sort_by(compare_edges_clockwise(&points[4], &points));
        assert_eq!(map_to(&edges), vec![7, 2, 5, 3, 6, 1, 8, 0]);
    }
}
