use super::Shape;
use std::{
    cmp::{PartialEq, PartialOrd}, fmt::{Debug, Display}, ops::{Add, Div, Mul, Sub}
};

pub struct Circle<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd + From<f64>
{
    radius: T,
    diameter: T,
}

impl<T> Shape<T> for Circle<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd + From<f64>
{
    fn area(&self) -> T {
        let pi = T::from(std::f64::consts::PI);
        pi * self.radius * self.radius
    }

    fn volume(&self) -> T {
        T::from(0.0)
    }
}