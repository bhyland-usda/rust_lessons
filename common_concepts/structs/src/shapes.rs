pub mod circle;
pub mod sphere;
pub mod cube;
pub mod prism;

pub use circle::Circle;
pub use sphere::Sphere;
pub use cube::Cube;
pub use prism::Prism;

pub trait Shape<T> {
    fn area(&self) -> T;
    fn volume(&self) -> T;
}