fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); 

    println!("The Length of string '{}' is {}", s1, len);

    println!("Mutability lives on!");
    living_on_mutability();
}


// This function does not take the string itself, but it
// takes the referene to the string. Opposite of referencing is dereferencing
// This makes sure the value is referenced but not owned. References as function
// parameters is called `borrowing`.
fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

// you can't change things that are borrowed. References are immutable by default
// Mutable references exist &mut value. But only one mutable reference can exist
// per scope. This prevents data racing but is annoying
// 1. two or more pointers access the same data at the same time.
// 2. At least one or more of the pointers is being used to write to the data
// 3. There is no mechanism being used to synchronize access to the data

// Curly brackets can be used to create new scope for mutation.
// mutable references cannot exist when we have immutable references.
// Multiple immutable references are as they are read-only references

fn living_on_mutability() {
    let mut s1 = String::from("Hello");

    let x = &s1;
    let y = &s1;
    println!("The immutable references are: x={} and y={}", x, y);

    let m = &mut s1;
    println!("This can happen because the scope has ended before: {}", m);

    // Errors in mutability and immutability are called borrowing errors
}

// Dangling pointer references a location in memory that may have been given to
// someone else by freeing some memory preserving a pointer to that memory