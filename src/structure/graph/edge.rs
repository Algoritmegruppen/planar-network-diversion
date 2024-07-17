use crate::structure::weight::{Weight, Weighted};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub trait Edge<W: Weight>:
    Weighted<W> + FromStr + Debug + Clone + PartialEq + Eq + PartialOrd + Ord
{
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn reverse(&self) -> Self;
    fn subdivide(&self, middle: usize) -> (Self, Self);
    fn shift_by(&self, offset: i64) -> Self;
}

pub fn map_to<W: Weight, E: Edge<W>>(edges: &Vec<E>) -> Vec<usize> {
    edges.iter().map(Edge::to).collect()
}

#[derive(PartialEq, Clone)]
pub struct BasicEdge<W: Weight> {
    from: usize,
    to: usize,
    weight: W,
}

impl<W: Weight> BasicEdge<W> {
    pub fn new(from: usize, to: usize, weight: W) -> Self {
        BasicEdge { from, to, weight }
    }
}

impl<W: Weight> Eq for BasicEdge<W> {}

impl<W: Weight> PartialOrd for BasicEdge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.from, self.to, self.weight).partial_cmp(&(other.from, other.to, other.weight))
    }
}

impl<W: Weight> Ord for BasicEdge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

impl<W: Weight> Edge<W> for BasicEdge<W> {
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
    fn reverse(&self) -> Self {
        BasicEdge {
            from: self.to,
            to: self.from,
            weight: self.weight,
        }
    }
    fn subdivide(&self, middle: usize) -> (Self, Self) {
        (
            BasicEdge {
                from: self.from,
                to: middle,
                weight: self.weight,
            },
            BasicEdge {
                from: middle,
                to: self.to,
                weight: 0.into(),
            },
        )
    }

    fn shift_by(&self, offset: i64) -> Self {
        BasicEdge {
            from: (self.from as i64 + offset) as usize,
            to: (self.to as i64 + offset) as usize,
            weight: self.weight,
        }
    }
}

impl<W: Weight> Debug for BasicEdge<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.weight == 1.into() {
            write!(f, "{} --> {}", self.from, self.to)
        } else {
            write!(f, "{} -{}-> {}", self.from, self.weight, self.to)
        }
    }
}

impl<W: Weight> Weighted<W> for BasicEdge<W> {
    fn weight(&self) -> W {
        self.weight
    }
}

impl<W: Weight> FromStr for BasicEdge<W> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rs = s.split(' ');
        let u = rs
            .next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let v = rs
            .next()
            .ok_or("Expected an unsigned integer here, but found nothing!")?
            .parse()
            .or(Err("Could not parse as an unsigned integer!"))?;
        let w = W::from_str(rs.next().unwrap_or_else(|| "1")).unwrap_or_else(|_| 1.into());

        Ok(BasicEdge {
            from: u,
            to: v,
            weight: w,
        })
    }
}
