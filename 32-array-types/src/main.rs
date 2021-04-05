use std::io;


fn main() {

    println!("Learning Arrays");

    let a = [1, 2, 3, 4, 5];

    println!("Enter an array index to access: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");
    
    let element = a[index];
    println!("The value of el at index {} is: {}", index, element);

    // Rusts protects against invalid array index operations by throwing this error.
    // In some languages, invalid indexes could be accessed causing some errors when
    // this is done.

}
