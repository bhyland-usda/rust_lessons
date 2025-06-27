// Import our Shape trait
use super::Shape;
// Import out utility math traits
use crate::utils::{FromFloat, ToFloat, Ops};
// Import the needed traits from the standard library
use std::{
    cmp::{PartialEq, PartialOrd}, 
    fmt::{Debug, Display}, 
    ops::{Add, Div, Mul, Sub},
};

/// Describes a Circle
pub struct Circle<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    radius: T,
    diameter: T,
}

impl<T> Circle<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    pub fn new_with_radius(radius: T) -> Self {
        Self {
            radius,
            diameter: radius * T::from_f64(2.0)
        }
    }

    pub fn new_with_diameter(diameter: T) -> Self {
        Self {
            diameter,
            radius: diameter / T::from_f64(2.0),
        }
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn diameter(&self) -> T {
        self.diameter
    }
}

/// Allow a Circle to return its area and volume properties
impl<T> Shape<T> for Circle<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    fn area(&self) -> T {
        let pi = T::from_f64(std::f64::consts::PI);
        pi * Ops::square(self.radius)
    }

    fn volume(&self) -> T {
        T::from_f64(0.0)
    }
}