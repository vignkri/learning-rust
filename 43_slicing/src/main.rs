fn main() {

    // slices do not have ownership
    let mut s = String::from("hello world");
    let output = first_word(&s);
    s.clear(); // empties the string and makes it ''

    println!("Length of input string is: {}", output);
    // the word is invalid but the length is still valid. :o

    // slicing strings
    let s = String::from("Hello worldipoop!");
    let output_two = slicing_strings(&s);
    println!("Output from slicing is: {}", output_two);

    // Returning sliced strings
    let mut input_s = String::from("Hello, this is a world of poop!");
    let output_sliced = returning_slices(&input_s);
    input_s.clear(); // can't clear throws mutability and immutability errors thanks
    // to how the slices are drawn. The function slices the input and creates a cleaned up
    // reference to the original data that goes in. Therefore, there is a clear effect on the 
    // system for the function to work correctly.
    println!("Input {} and output is {}", input_s, output_sliced);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // this moves through each element, we convert it into bytes
    // these are iterators on the array. Enumerate wraps the iterator
    // and returns them as a tuple

    for (i, &item) in bytes.iter().enumerate() {
        // for each value we search for a space in the bytes value
        // Enumerate returns tuples which is the i and the byte i.
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn slicing_strings(s: &String) -> &str {

    let len = s.len();
    
    let hello = &s[0..5]; // Starts at the first [0..2] is equal to [..2]
    let world = &s[6..len]; // 6 to length is basically the same as [6..12] == [6..]

    world
}


fn returning_slices(s: &String) -> &str {
    let input_as_bytes = s.as_bytes();

    for (i, &item) in input_as_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn slices_as_params(s: &str) -> &str {
    // takes slices as the input parameter
}