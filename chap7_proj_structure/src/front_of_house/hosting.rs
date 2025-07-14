//bring rand module into scope
use rand::Rng;

pub fn add_to_waitlist() {
    let wailist_number = rand::rng().random_range(1..=100); // Generates a random number between 1 and 100
    println!("Added to the waitlist. Your number is: {}", wailist_number);    
}   