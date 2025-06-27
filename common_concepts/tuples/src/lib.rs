/*
    Tuples are a bunch of values lumped together. The types do not have to be the same.
    
    The cool thing about tuples is that you get one from a function and deconstruct it
    to have multiple useful variables for later use. Many functions of the standard library
    actually use this to their advantage to return several values of different types.

    Examples:
        let tuple = (42i32, "&str", 100.10f64);
        let str_tuple: (&str, &str, &str) = ("Hello,", " there", " guy!");
*/

#[cfg(test)]
mod tests {
    // Function to test
    fn return_tuple() -> (&'static str, String, f32) {
        let the_answer = ("The ", "answer is:".to_string(), 42.0);

        the_answer
    }

    // Test the function
    #[test]
    fn return_tuple_test() {
        let (the, answer_is, fourty_2) = return_tuple();
        assert!(the == "The ");
        assert!(answer_is == String::from("answer is:"));
        assert!(fourty_2 == 42.0);
    }
}
