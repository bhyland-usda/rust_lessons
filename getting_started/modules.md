# Rust Modules Made Easy
Modules are parts of a crate outside of main.rs/lib.rs file. It's how crates are organize the codebase to make it easier to work with. There are
three ways that we can split things into modules:

1. All files are in the src directory.
2. Directories within the src directory that have a .rs file of the same name in the src directory.
3. Directories within the src directory that have a mod.rs file within them.

## All Files in src Directory
This is the most straight forward way of doing this, and is the preferred way if your crate/project is very small. It would look something like:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils.rs  
  
With this method the only place you need to declare your module (i.e.; mod module_name;) is at the top of main.rs (or lib.rs if it's a library).  

### Example
```rust
// main.rs

pub mod utils;

use utils::math::{add, subtract};

fn main() {
    let four = add(2, 2);
    let three = subtract(four, 1);

    println!("2 + 2 = {four}");
    println!("4 - 1 = {three}");
}
```

## Directories with their .rs Files
This is the modern Rust way of doing things for crates/projects that are larger and likely to get unwieldy doing it with all of the source files in the src directory.
It looks something like the following:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils.rs  
|&nbsp;&nbsp;&nbsp;-- utils  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|    
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- math.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- bounds.rs  
|&nbsp;&nbsp;&nbsp;-- shapes.rs  
|&nbsp;&nbsp;&nbsp;-- shapes  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- circle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- rectangle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- triangle.rs  
  
Unlike the previous method, you have to add `mod module_name;` to the top of main.rs or lib.rs, but within the module_name.rs file you also have to add `mod file_name;` at the top
for every file within the directory. This way all of the modules are declared for the project and the compiler can see them.  

### Example
```rust
// main.rs

pub mod utils;
mod shapes;

use shapes::{circle::Circle, rectangle::Rectangle, triangle::Triangle};

fn main() {
    let c = Circle::new_from_radius(5);

    println!("The area of the circle is: {}", c.area());
}
```  

```rust
// shapes.rs
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
```  

```rust
// circle.rs
use super::Shape;
use crate::utils::{FromFloat, ToFloat, Ops};
use std::{
    cmp::{PartialEq, PartialOrd}, fmt::{Debug, Display}, ops::{Add, Div, Mul, Sub}
};

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
```

## Directories with mod.rs Files
This is the older way of doing things, but it still works fine and sometimes can actually be cleaner looking. The down side is that there are multiple files named `mod.rs` in
your project and can be confusing which one you're on if you have a lot of them. This method structure looks similar to the following:  
  
project_dir  
|  
--- src  
|&nbsp;&nbsp; |  
|&nbsp;&nbsp;&nbsp;-- main.rs  
|&nbsp;&nbsp;&nbsp;-- utils  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- mod.rs    
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- math.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- bounds.rs  
|&nbsp;&nbsp;&nbsp;-- shapes  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- mod.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- circle.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- cube.rs  
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-- triangle.rs  
  
Just like the previous method, this method requires you to do `mod module_name;` at the top of main.rs/lib.rs as well as `mod file_name;` at the top of each mod.rs file for every file you
create in the directory.  
  
### Example
```rust
// main.rs

pub mod utils;
mod shapes;

use shapes::{circle::Circle, cube::Cube};

fn main() {
    let c = Circle::new_from_diameter(10);

    let cube = Cube::new(3);

    println!("The area of the circle is: {}", c.area());
    println!("The area of the cube is: {}", cube.area());
}
```  

```rust
// src/shapes/mod.rs
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
```  

```rust
// shapes/cube.rs
use std::{
    cmp::{PartialEq, PartialOrd}, 
    fmt::{Debug, Display}, 
    ops::{Add, Div, Mul, Sub},
};
use super::Shape;
use crate::utils::{FromFloat, Ops, ToFloat};

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
```
---  
  
## Why Use Modules?
No matter which way you choose to declare modules for your project, it's better to have similar functionality chunked into smaller modules rather than everything in one single file, especially
not having everything in main.rs.