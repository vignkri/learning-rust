fn main() {
    // Control flows allow for controling the flow of information

    let number = 6;

    if number < 5 {
        println!("Number is true");
    } else {
        println!("Number is false");
    }

    // the value needs to be a bool for the control below
    // to work correctly
    // if number {
    //     println!("Condition is not a bool.");
    // }

    // Multiple if conditions
    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Divisible not by 4, 3, 2.");
    }

    // Using if in a let statement
    let condition = false;
    let n_number = if condition {1} else {6};  // values in the if-arm should be of the same type
    // These values can't evaluate to different types
    println!("Number is {}", n_number);

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The counter counts {} and the result is {}", counter, result);

    // Conditional loops with thee data
    let mut number = 3;
    
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // Looping through collections
    let a = [10, 20, 30, 40];
    let mut index = 0;

    while index < 4 {
        println!("The value is {}", a[index]);
        index += 1;
    }

    // Concise collection iteration 
    println!("Collection iteration concise using `iter()` ");
    for element in a.iter() {
        println!("The value is {}", element);
    }

    // Reverse a range using the rev function
    println!("Reversing a range using the rev function: ");
    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("Non reversed range");
    // Range can be called using start_number .. end_number
    // This is fast way of iterating through the numbers
    for number in 1..10 {
        println!("{}", number);
    }
}
