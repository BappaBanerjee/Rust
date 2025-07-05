use std::io;
use rand::Rng; // Importing the Rng trait from the rand crate
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100); // Generates a random number between 1 and 100
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop if the guess is correct
            }
        }
    }
    
    
}
