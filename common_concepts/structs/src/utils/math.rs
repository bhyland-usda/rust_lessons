use std::{
    cmp::{PartialEq, PartialOrd}, fmt::{Debug, Display}, marker::PhantomData, ops::{Add, Div, Mul, Sub}
};

pub struct Ops<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
{
    phantom: PhantomData<T>,
}

impl<T> Ops<T>
where T:
    Clone + Copy + Debug + Display + Add<Output=T> + Sub<Output=T> 
    + Mul<Output=T> + Div<Output=T> + PartialEq + PartialOrd
    + FromFloat + ToFloat
{
    pub fn square(prop: T) -> T {
        prop * prop
    }

    pub fn half(value: T) -> T {
        value / FromFloat::from_f64(2.0)
    }

    pub fn cube(value: T) -> T {
        value * value * value
    }

    pub fn double(value: T) -> T {
        value * FromFloat::from_f64(2.0)
    }
}

pub trait ToFloat {
    fn to_f64(self) -> f64;
}

impl ToFloat for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToFloat for i64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToFloat for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToFloat for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToFloat for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToFloat for u64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

pub trait FromFloat {
    fn from_f64(f: f64) -> Self;
}

impl FromFloat for i32 {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}

impl FromFloat for i64 {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}

impl FromFloat for usize {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}

impl FromFloat for u32 {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}

impl FromFloat for u64 {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}

impl FromFloat for f32 {
    fn from_f64(f: f64) -> Self {
        f as Self
    }
}