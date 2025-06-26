/*
    There is a concrete array type built into Rust. However, it is more common to use a Vector than
    it is to use an array. Arrays should only be used when you absolutely know that the size of the
    the collection is going to be and that size does not need to change, as arrays cannot grow or
    shrink after declared. The most common way to use an array is in the context of a byte stream,
    which is a [u8].

    You'll notice that in the test function that an array needs to be wrapped in a type like a
    Box. This is because an array is stack allocated and its size needs to be known at compile time.
    Because the compiler cannot determine that size at compile time for it to return the array from
    the function it throws an error and will not compile if you try returning an array without being 
    wrapped. The Box type will be discussed at a later time when smart pointers are gotten to, but 
    just know for now that Box allocates the type on the Heap and the size is known at the time of 
    compilation, allowing for a successful compilation.

    Declaration Examples:
        let a = [1, 2, 3, 4, 5];
        let b: [String, 3] = [String::from("Hello"), String::from("there"), "guy".to_string()];
        let zeros = [0; 10]; <- Initialized the array with 10 elements all with the value of 0.
*/

fn give_me_ten_zeros() -> Box<[u32]> {
    Box::new([0; 10])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_me_ten_zeros() {
        give_me_ten_zeros().iter().for_each(|ele| {
            assert_eq!(*ele, 0);
        });
    }
}
