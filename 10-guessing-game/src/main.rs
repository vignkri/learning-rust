use std::io;
use std::cmp::Ordering;
use rand::Rng;  // This is a trait

// gen_range from Rng is inclusive lower and exclusive upper bounds

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        // let guess: u32 = guess.trim().parse().expect("Please type a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to write a number.");
                continue;
            }
        };  // Expect to match allows us to handle error states better.

        println!("The secret number is: {}", secret_number);  
  
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

    }

}
