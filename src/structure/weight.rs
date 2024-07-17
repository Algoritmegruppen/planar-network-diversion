use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Sub};
use std::str::FromStr;

pub trait Weight:
    Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Clone
    + PartialEq
    + FromStr<Err: Display + Debug>
    + From<u32>
    + PartialOrd
    + Copy
    + Default
    + Debug
    + Display
{
}

impl<T> Weight for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Clone
        + PartialEq
        + FromStr<Err: Display + Debug>
        + From<u32>
        + PartialOrd
        + Copy
        + Default
        + Debug
        + Display
{
}

#[derive(PartialEq, PartialOrd)]
pub struct Order<T: PartialOrd + PartialEq>(pub T);

impl<T: PartialOrd + PartialEq> Ord for Order<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}
impl<T: PartialOrd + PartialEq> Eq for Order<T> {}

pub trait Weighted<W: Weight> {
    fn weight(&self) -> W;
}
