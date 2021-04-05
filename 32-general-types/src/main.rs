fn main() {

    println!("Compound Types: Tuples");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Tuple is: {:?}", tup); // special printer for printing complex types
    println!("Tuple unpacking.");

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);  // This is called destructuring of a tuple

    let pos_one = tup.0;
    println!("Position 0: {}", pos_one);  // Access indices by getting the first position. Starts with 0;

    
    println!("Compound Types: Arrays");
    // Allows data to be stored in a stack rather than a heap
    // Arrays cannot be grown or shrunk in size. They are not as flexible as vectors;
    // Unsure to use array / vector -> use vector
    let _arr_one = [1, 2, 3, 4, 5];

    // use type and length in the array specifier
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Find the value at pos0 using [0]: {}", _arr[0]);  // Tuple indexing is different from array indexing

    // specify an array in life using the array specified
    let throughout = [1; 5];  // this sets all the values to 1 and the array length is 5
    println!("Length of array: {}", throughout.len());

    println!("Invalid access of array");
}
