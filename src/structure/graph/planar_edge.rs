use crate::structure::graph::edge::Edge;
use crate::structure::graph::point::Point;
use crate::structure::weight::{Weight, Weighted};
use std::cmp::Ordering::{self, Equal};
use std::fmt::{Debug, Formatter, Display};
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub struct AbstractPlanarEdge<W: Weight, S: Sealed> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub(in crate::structure::graph) left: S,
    pub(in crate::structure::graph) right: S,
}

pub trait Sealed: PartialEq + PartialOrd + Copy + Default {}
impl Sealed for usize {}
impl Sealed for Option<usize> {}

pub type PlanarEdge<W> = AbstractPlanarEdge<W, usize>;
pub(in crate::structure::graph) type PrePlanarEdge<W> = AbstractPlanarEdge<W, Option<usize>>;

impl<W: Weight, S: Sealed> AbstractPlanarEdge<W, S> {
    pub fn format_with_coords(&self, points: &Vec<Point>) -> String {
        let a = points[self.from()];
        let b = points[self.to()];
        format!("({:.1},{:.1}) <===> ({:.1},{:.1})", a.x, a.y, b.x, b.y)
    }
}

impl<W: Weight> PlanarEdge<W> {
    pub fn left(&self) -> usize {
        self.left
    }
    pub fn right(&self) -> usize {
        self.right
    }
    pub fn rotate_right(&self) -> Self {
        PlanarEdge {
            from: self.left,
            to: self.right,
            left: self.to,
            right: self.from,
            weight: self.weight,
        }
    }
}

impl<W: Weight> PrePlanarEdge<W> {
    pub fn planarize(&self) -> PlanarEdge<W> {
        PlanarEdge {
            from: self.from,
            to: self.to,
            weight: self.weight,
            left: self.left.unwrap(),
            right: self.right.unwrap(),
        }
    }
    pub const fn new(from: usize, to: usize, weight: W) -> Self {
        PrePlanarEdge {
            from,
            to,
            weight,
            left: None,
            right: None,
        }
    }
}

pub use intersection::intersect;
mod intersection {
    use crate::structure::graph::edge::Edge;
    use crate::structure::graph::point::Point;
    use crate::structure::weight::Weight;
    use std::cmp::Ordering::{Equal, Greater, Less};
    use Orientation::*;

    #[derive(PartialEq)]
    enum Orientation {
        Clockwise,
        Counterclockwise,
        Colinear,
    }

    pub fn intersect<W: Weight, E: Edge<W>>(points: &Vec<Point>, ab: &E, cd: &E) -> bool {
        let a = &points[ab.from()];
        let b = &points[ab.to()];
        let c = &points[cd.from()];
        let d = &points[cd.to()];

        if a == c || a == d || b == c || b == d {
            return false;
        }

        let o1 = orientation(a, b, c);
        let o2 = orientation(a, b, d);
        let o3 = orientation(c, d, a);
        let o4 = orientation(c, d, b);

        o1 != o2 && o3 != o4
            || o1 == Colinear && on_segment(a, c, b)
            || o2 == Colinear && on_segment(a, d, b)
            || o3 == Colinear && on_segment(c, a, d)
            || o4 == Colinear && on_segment(c, b, d)
    }
    fn orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        match val.total_cmp(&0.0) {
            Greater => Clockwise,
            Less => Counterclockwise,
            Equal => Colinear,
        }
    }

    fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
    }
}

impl<W: Weight, S: Sealed> Weighted<W> for AbstractPlanarEdge<W, S> {
    fn weight(&self) -> W {
        self.weight
    }
}

impl<W: Weight, S: Sealed> Debug for AbstractPlanarEdge<W, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.weight, self.to)
    }
}
impl<W: Weight, S: Sealed> Display for AbstractPlanarEdge<W, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.weight, self.to)
    }
}

impl<W: Weight, S: Sealed> PartialOrd for AbstractPlanarEdge<W, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.from, self.to, &self.left, &self.right, self.weight).partial_cmp(&(
            other.from,
            other.to,
            &other.left,
            &other.right,
            other.weight,
        ))
    }
}

impl<W: Weight, S: Sealed> Ord for AbstractPlanarEdge<W, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

impl<W: Weight, S: Sealed> Edge<W> for AbstractPlanarEdge<W, S> {
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
    fn reverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }

    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            Self {
                from: self.from,
                to: middle,
                weight: self.weight,
                left: self.left,
                right: self.right,
            },
            Self {
                from: middle,
                to: self.to,
                weight: 0.into(),
                left: self.left,
                right: self.right,
            },
        )
    }
    fn shift_by(&self, offset: i64) -> Self {
        Self {
            from: (self.from as i64 + offset) as usize,
            to: (self.to as i64 + offset) as usize,
            weight: self.weight,
            left: self.left,
            right: self.right,
        }
    }
}

impl<W: Weight, S: Sealed> Eq for AbstractPlanarEdge<W, S> {}

impl<W: Weight, S: Sealed> FromStr for AbstractPlanarEdge<W, S> {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rs = s.split(' ');
        Ok(Self {
            from: rs
                .next()
                .ok_or("Expected an unsigned integer here, but found nothing!")?
                .parse()
                .or(Err(
                    "Could not parse the base of the edge as an unsigned integer!",
                ))?,
            to: rs
                .next()
                .ok_or("Expected an unsigned integer here, but found nothing!")?
                .parse()
                .or(Err(
                    "Could not parse the tip of the edge as an unsigned integer!",
                ))?,
            weight: W::from_str(rs.next().unwrap_or_else(|| "1")).unwrap_or_else(|_| 1.into()),
            left: S::default(),
            right: S::default(),
        })
    }
}

#[cfg(test)]
mod test_intersection {
    use crate::structure::graph::planar_edge::{intersect, PrePlanarEdge};
    use crate::structure::graph::point::Point;

    const POINTS: [Point; 4] = [
        Point::new(0.0, 5.0),
        Point::new(5.0, 5.0),
        Point::new(0.0, 0.0),
        Point::new(5.0, 0.0),
    ];

    const EDGES: [PrePlanarEdge<u64>; 6] = [
        PrePlanarEdge::new(0, 1, 0),
        PrePlanarEdge::new(0, 2, 0),
        PrePlanarEdge::new(0, 3, 0),
        PrePlanarEdge::new(1, 2, 0),
        PrePlanarEdge::new(1, 3, 0),
        PrePlanarEdge::new(2, 3, 0),
    ];

    fn assert_intersect(points: &Vec<Point>, expected: bool, i: usize, j: usize) {
        let ab = &EDGES[i];
        let cd = &EDGES[j];
        assert_eq!(expected, intersect(points, ab, cd), "{:?} x {:?}", ab, cd);
    }

    #[test]
    fn test_intersection() {
        let points = Vec::from(POINTS);
        assert_intersect(&points, false, 0, 1);
        assert_intersect(&points, false, 0, 2);
        assert_intersect(&points, false, 1, 3);
        assert_intersect(&points, false, 0, 5);

        assert_intersect(&points, true, 2, 3);
    }
}
