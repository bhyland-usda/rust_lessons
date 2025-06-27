pub mod circle;
pub mod rectangle;
pub mod triangle;
pub mod sphere;
pub mod cube;
pub mod pyramid;

pub use circle::Circle;
pub use rectangle::Rectangle;
pub use triangle::Triangle;
pub use sphere::Sphere;
pub use cube::Cube;
pub use pyramid::Pyramid;

pub trait Shape<T> {
    fn area(&self) -> T;
    fn volume(&self) -> T;
}