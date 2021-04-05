
// const MAX_POINTS: u32 = 100_000; // consts are available on the scope throughout the program
// depending on the scope they are available to the entire scope
// Constants are immutable. These can't be defined as a mutable constant

fn main() {

    let x: u32 = 5;
    println!("The value of x: {}", x);

    let x = x + 1;

    let x = x * 2;

    println!("The value of x: {}", x);
    // let makes the value to be shadowed. Will throw mutability error if `let` is not
    // used. This allows for mutability while doing certain transformations and become
    // immutable after these are performed

    // mutables can't be allowed to change types
    // let mut spaces = "    ";
    // spaces = spaces.len();  // this will not work and throw an error for type changing

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {}", spaces);
}
