use std::{
    cmp::{PartialEq, PartialOrd}, 
    fmt::{Debug, Display}, 
    ops::{Add, Div, Mul, Sub},
};
use super::Shape;
use crate::utils::{FromFloat, Ops, ToFloat};

/// Describes a Cube
pub struct Cube<T> 
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    sides: T,
}

impl<T> Cube<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    pub fn new(sides: T) -> Self {
        Self {
            sides
        }
    }

    pub fn sides(&self) -> T {
        self.sides
    }
}

/// Allow a Cube to return its area and volume properties
impl<T> Shape<T> for Cube<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T>
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    fn area(&self) -> T {
        T::from_f64(6.0) * Ops::square(self.sides)
    }

    fn volume(&self) -> T {
        Ops::cube(self.sides)
    }
}