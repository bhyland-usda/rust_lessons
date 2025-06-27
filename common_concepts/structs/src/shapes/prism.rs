use std::{
    cmp::{PartialEq, PartialOrd}, 
    fmt::{Debug, Display}, 
    ops::{Add, Div, Mul, Sub},
};
use super::Shape;
use crate::utils::{FromFloat, Ops, ToFloat};

/// Describes a Prism (3D Box)
pub struct Prism<T> 
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    length: T,
    width: T,
    height: T,
}

impl<T> Prism<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T>
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    pub fn new(length: T, width: T, height: T) -> Self {
        Self {
            length,
            width,
            height
        }
    }

    pub fn length(&self) -> T {
        self.length
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

/// Allow a Prism (Box) to return its area and volume properties
impl<T> Shape<T> for Prism<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T>
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    fn area(&self) -> T {
        let h_w = self.height * self.width;
        let w_l = self.width * self.length;
        let h_l = self.height * self.length;
        Ops::double(h_w + w_l + h_l)
    }

    fn volume(&self) -> T {
        self.length * self.width * self.height
    }
}