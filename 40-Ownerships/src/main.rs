// Memory is managed through ownership with a set of rules that compiler checks at compile time
// Rules of Ownership
// 1. Each value in rust has a variable that is called its owner
// 2. There can only be one owner at a time
// 3. When an owner goes out of scope, the value is dropped
// Variable scope

fn main() {

    println!("Variable Scoping: ");
    variable_scoping();

    println!("Variable moving: ");
    variable_borrow();

    println!("Variable cloning: ");
    variable_cloning();

    // In case of integers, the cloning and borrowing are not affected.
    // The reason is that the size is known at runtime based on the type. Strings
    // are different as they are on the heap. Known size types are in the stack
    // instead of the heap

    // For these cases rust has a special trait called `Copy` on types like integers
    // that are stored on the stack. Types with copy trait are: bools, all integers,
    // floating points, character types, tuples

    println!("Ownership: ");
    ownerships();

    println!("Returning Ownerships: ");
    return_values_and_scopes();

} 

fn return_values_and_scopes() {
    let s1 = gives_ownership(); // moves the s1 from the return statement to s

    let s2 = String::from("hello"); // s2 comes into the scope
    let s3 = takes_and_gives_back(s2); // moves it into scope and returns back from scope
    // when a variable iin the heap goes out of scope, the value is cleaned up by the `drop`
    // usually this causes a complicated effect of handling returns and inputs
    // to overcome this borrowing / moving tediousness, Rust has references
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    a_string
}



fn ownerships() {
    let s = String::from("hello");
    takes_ownership(s);  // s goes out of scope thanks to ownership being transferred to the function
    // so S doesn't exist here in the scope

    // println!("{}", s);

    let x = 5;
    takes_integer_ownership(x); // here there is a copy thanks to the variable being an integer
    // so, it can be handled in this scope as well
    println!("{}", x);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_integer_ownership(i: u32) {
    println!("{}", i);
}


fn variable_cloning() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();  // copying of the heap data
    println!("{} and {}", s1, s2); // This is expensive behavior in the code due to copying

}


fn variable_scoping() {
    let mut s = String::from("hello");  // double-colon allows us to namespace this particular from
    // requests the memory it needs when using string::from
    // we can return the memory when s goes out of scope
    // each allocated needs to be paired with exactly one `free` action

    s.push_str(", World!");  // push on a string appends a literal to the string

    println!("{}", s);  

    // Rust calls drop on the string memory whenever the curly brackets are hit
} // s becomes invalid as the scope is lost


fn variable_borrow() {

    let s1 = String::from("Hello");
    let s2 = s1; // when something is copied, we move the ptr, length and the capacity on the stack
    // this means that the heap data is not copied.
    println!("{}", &s2);

    // when s1 and s2 are going out of scope, they will both try to free the same memory
    // known as `double free` error and is a memory safety bug
    // Calling s1 will cause invalidated error issue as the value has been referenced elsewhere by s2
    // Rust also invalidates the first variable and therefore is called a `move`
    // there are no deep copies.
}