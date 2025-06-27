/*
    There are multiple types of strings in Rust. The two basic types are:

    String slices (&str): Also known as string literals.
        Examples: 
            let string_slice: &str = "This is a string slice";
            let string_slice = "This is also a string slice";
            let raw_string = r##"This is a string #"literal#"."##;   => Can escape characters with # in this usage. 

    String (String): This is actually a struct rather than a concrete type.
        Examples:
            let s: String = String::new(); -> Creates and empty String.
            let s = String::from("This is a String");
            let s = "This is also a string".to_string();
            let num_string = 1.to_string(); -> Converts the number into a String
            let gravity_law = format!("What goes \u{2191} must come \u{2193}!");

    Char (char): A single utf-8 character; declared with the single quote.
        Examples:
            let c: char = 'c';
            let a = 'a';
            let up_arrow = '\u{2191}'; -> Standard UTF-8 unicode for the up arrow symbol.
            let down_arrow = '\u{2193}'; -> Standard UTF-8 unicode for the down arrow symbol.

    Less commonly used string types:        
        - Byte String: Byte strings are like string literals/slices, but are converted to a u8 (byte) value that contains the string itself.
                       The string literal is considered a string of bytes by the compiler vs. treating it as a string.
            
            Example:
                let byte_string = b"This is a byte string, which means really the type is a [u8, string_length]";

*/

#[cfg(test)]
mod tests {
    #[test]
    fn gravity_law_test() {
        let gravity_law = || {
            String::from("What goes \u{2191} must come \u{2193}!")
        };

        assert_eq!(gravity_law(), "What goes ↑ must come ↓!".to_string());
    }
}
