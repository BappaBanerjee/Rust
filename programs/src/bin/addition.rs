use std::io;

// cargo run --bin addition
fn main(){
    let mut first_number = String::new();
    let mut second_number = String::new();

    io::stdin().read_line(&mut first_number).expect("cannot read input");
    let a : i32 = first_number.trim().parse().expect("invalid number");

    io::stdin().read_line(&mut second_number).expect("cannot read input");
    let b : i32 = second_number.trim().parse().expect("invalid number");
    
    let sum = a + b;
    println!("sum is : {sum}");
}