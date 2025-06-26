/*
    Declaring variables in Rust is fairly straight forward.

    Declaring them always starts with the keyword `let`, this is the
    keyword that tells the compiler that we're declaring a new variable.

    The next part that always comes around is the variable name.

    Next there are options: `implicit typing` and `explicit typing`.

    - Implicit Typing:
        This is where we let the compiler assign the default or context driven type to the variable.
        The compiler is smart enough to figure out what type a variable should be, in most cases, by
        looking at the code AFTER the variable declaration. Most of the time, implicit typing is what
        is used to declare a variable.

        - Examples: 
            let my_int = 12;
            let my_string = String::new();
            let my_other_string = "I am not an empty string like the one above me".to_string();
            let my_str = "New str";
            let my_vec = Vec::new(); 
    
    - Explicit Typing:
        This is where we explictly tell the compiler what type the variable should be at declaration time.
        It follows the syntax of `let var_name: Type = value;` or `let var_name = valueType`. This can be required if the compiler can't
        figure out what you want this variable to be. A common case of this is an iterator using the .collect()
        method to return a collection of some sort, or something being returned from the .map() method that the
        compiler wouldn't expect.

        - Examples:
            let my_usize: usize = 42;
            let my_str: String = String::from("This is a String, not a &str!");
            let my_string: &str = "This is a &str not a String!";
            let my_struct_collection: Vec<CustomStruct> = vec![CustomStruct{ value: 1, }, CustomStruct{ value: 2, }]; -> This could also be: ... = Vec::new();
            let my_u8 = 42u8; -> Notice here that I didn't do the type declaration in the signature, but in the value.
                NOTICE: The value type declaration only works for numerical types such as integers and floats.

    One thing that can throw a wrench into the given structure above is mutability. I'll go more into that in the mutability section, but you must know
    about it when declaring variables as well.

    All variables are immutable by default. So, if you want to be able to change a variable you have to declare it mutable when you declare it.

        - Mutability Declaration Examples:
            let mut my_usize = 42usize;
            my_usize = 12;

            let mut my_string: String = "This is a string".to_string();
            my_string = String::from("Still a string");
*/

fn declare_ints() -> (i64, i32) {
    // Implicit declaration
    let int64 = 42;
    // Explicit declaration
    let int32 = 32i32;
    
    // Returning this successfully proves the variables were declared as the types specified.
    (int64, int32)
}

fn declare_string_types() -> (&'static str, String) {
    // Explicit declaraton
    let ret_str: &str = "This is the &str";
    // Implicit declaration
    let ret_string = String::from("This is the string");

    // Returning this successfully proves the variables were declared as the types specified.
    (ret_str, ret_string)
}

// Doesn't return anything because changes the elements in place.
fn declare_mutable_string_vec(input: &mut Vec<String>) {
    // Create a mutable i64 variable, implicitly
    let mut count = 0;

    // Loop through each element of the input Vec and change
    // it to the count.
    input.into_iter().for_each(|ele| {
        // Dereference the element and change the value to count as a String
        *ele = count.to_string();
        // Update the count
        count += 1;         
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_ints_test() {
        // Declare a tuple variable of (i64, i32)
        let ret_ints = declare_ints();

        // Prove those are the types was gotten back from the function
        assert_eq!(ret_ints, (42i64, 32i32))
    }

    #[test]
    fn declare_string_types_test() {
        // Declare a tuple variable of (&str, String)
        let ret_strs: (&str, String) = declare_string_types();

        // Prove that the variables and their types returned from the function are what is expected.
        assert_eq!(("This is the &str", String::from("This is the string")), ret_strs);
    }

    #[test]
    fn declare_mutable_string_vec_test() {
        // Vec variable declaration is mutable
        let mut input = vec!["String 1".to_string(), "String 2".to_string()];

        // Prove that the elements are what was declared
        assert_eq!(input[0], "String 1".to_string());
        assert_eq!(input[1], "String 2".to_string());

        // Change the elements to their index within the Vec
        declare_mutable_string_vec(&mut input);

        // Prove that they were changed to their index
        assert_eq!(input[0], "0".to_string());
        assert_eq!(input[1], "1".to_string());
    }
}
