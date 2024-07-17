use crate::structure::graph::edge::Edge;
use crate::structure::graph::planar_edge::PrePlanarEdge;
use crate::structure::weight::Weight;
use std::cmp::Ordering::{self, Equal};
use std::ops::{Add, Sub};

#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    fn angle(&self) -> f64 {
        self.y.atan2(self.x)
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
