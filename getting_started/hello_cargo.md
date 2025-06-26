# Getting Started with Cargo

---

## Create a New Crate/Project
### Binary Project  
```bash
cargo new --bin project_name

# or

cargo new project_name
```
  
For a binary project, you can leave out the `--bin` as it's implicit that that's what you want if the flag is left out.  
  
### Library Project 
*A library/crate project is one for other developers to consume and use the code, it does not have its own executable file. Think a framework of some sort that you import.*  
```bash
cargo new --lib project_name
```  
**Note**: *The `cargo new` command must have the `--lib` flag in it to create a libary crate!*  
  
---  
  
## Creating Tests - Easy Mode
The easiest way, as there are many, to create an integration test in Rust is to create a test module within the file that you're working on. This way the tests are with the code that they are testing. To do this you do the following in the file that your code is in:
```rust
#[cfg(tests)]
mod tests {
   use super::*; // or use super::{function_name, other_function_name}; or use super::function_to_test;

   #[test]
   fn function_name_test() {
       // ... Do the function testing and asserts ...
   }
}
```  
  
After writing your tests, you should probably run them, and run them fairly often if you're changing a lot of things.  
```bash
cargo test
```  
  
This will give you an output of the tests including passing and failing tests.  
  
---
  
## Creating Documentation
In Rust we have tools to automatically generate documentation websites for our projects. First you have to put documentation comments to document your code.
```rust
/// Add adds to i32 variables together and returns the sum
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```  
  
Then, you run the `cargo doc` command and it will generate the documentation; placing it in the `target/doc` directory.
