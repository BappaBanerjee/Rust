use std::io;

fn main() {
    println!("Hello, world!");
    let result = add();
    println!("The sum is: {}", result);
}

fn add() -> i32 {
    let mut first_num = String::new();
    let mut second_num = String::new();

    println!("Please enter the first number:");
    io::stdin()
        .read_line(&mut first_num)
        .expect("failed to read the input");
    println!("Please enter the second number:");
    io::stdin()
        .read_line(&mut second_num)
        .expect("failed to read the input");

    let a: i32 = first_num
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let b: i32 = second_num.trim().parse().expect(
        "Please enter a valid
number",
    );
    a + b
}
