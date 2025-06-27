use std::f64;

use crate::shapes::Shape;

pub struct Sphere<T> {
    radius: T,
    diameter: T,
}

impl<T> Sphere<T> {
    pub fn new_with_radius(radius: T) -> Self {
        Self {
            radius,
            diameter: radius * 2,
        }
    }

    pub fn new_with_diameter(diameter: T) -> Self {
        Self {
            diameter,
            radius: diameter / 2,
        }
    }
}

impl Shape<f64> for Sphere<f64> {
    fn area(&self) -> f64 {
        4.0 * f64::consts::PI * self.radius.exp2()
    }

    fn volume(&self) -> f64 {
        4/3 * f64::consts::PI * self.radius^3
    }
}