use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub, Mul, Div},
    cmp::{PartialEq, PartialOrd},
    f64::consts::PI,
};

use super::Shape;
use crate::utils::{FromFloat, ToFloat, Ops};

/// Describes a Sphere
pub struct Sphere<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    radius: T,
    diameter: T,
}

impl<T> Sphere<T> 
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    pub fn new_with_radius(radius: T) -> Self {
        Self {
            radius,
            diameter: radius * FromFloat::from_f64(2.0),
        }
    }

    pub fn new_with_diameter(diameter: T) -> Self {
        Self {
            diameter,
            radius: diameter / FromFloat::from_f64(2.0),
        }
    }

    pub fn diameter(&self) -> T {
        self.diameter
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn circumference(&self) -> T {
        T::from_f64(2.0) * T::from_f64(PI) * self.radius
    }
}

/// Allow a Sphere to return its area and volume properties
impl<T> Shape<T> for Sphere<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    fn area(&self) -> T {
        T::from_f64(4.0) * FromFloat::from_f64(PI) * Ops::<T>::square(self.radius)
    }

    fn volume(&self) -> T {
        T::from_f64(4.0/3.0) * FromFloat::from_f64(PI) * Ops::<T>::cube(self.radius)
    }
}