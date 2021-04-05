fn main() {
    println!("Hello, world!");

    another_function();

    // calling a function with parameters
    with_parameters(5, 6);

    // calling functions with statements
    with_statements();

    // expressions evaluate to something
    let y = {
        let z = 3;
        z + 1
    };  // this evaluates to 4 and therefore can be assigned
    println!("Evaluation of expressions: {}", y);

    // Evaluation of functions with return values
    let r: i32 = with_returns();
    println!("Returned value is {}", r);

    // Evaluation of a function with params
    let res: i32 = plus_one(5);
    println!("Returned value is {}", res);

}

// Function definitions need not be defined above, can be anywhere

fn another_function() {
    println!("This is another function!");
}

// Parameters are defined and these are arguments to the functions
// function signatures must have declared types of each parameter
// Each parameter needs to have separate definitions

fn with_parameters(x: i32, y: i32) {
    println!("The value of x is {} and y is {}", x, y);
}


// Defining statements within a particular function

fn with_statements() {
    let _m = 6;  // statements do not return values and so you can't assign them to variables

    // let m = (let x = 6);  // this is valid in C/Ruby but not in rust
}

// Functions with return statements

fn with_returns() -> i32 {
    5
}

// Functions with operations

fn plus_one(x: i32) -> i32 {
    x + 1  // return statements do not have semicolons
    // statements that do not create a value evaluate to an empty tuple
    // this causes issues with the compilers
}
