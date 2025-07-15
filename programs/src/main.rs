use std::fmt::Display;
fn main() {
    println!("Hello, world!");
    //call a function to print the generic params
    print_generic_params(42, "Hello");
}

fn print_generic_params<T : Display, U : Display>(param1: T, param2: U) {
    println!("First parameter: {}", param1);
    println!("Second parameter: {}", param2);
}

