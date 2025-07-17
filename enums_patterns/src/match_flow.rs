pub fn match_flow() {
    // Enums in Rust
    // Enums are a powerful feature in Rust that allows you to define a type that can be one of several different variants.
    // Each variant can have its own data and behavior, making enums a versatile tool for modeling complex data structures.

    // Example of a simple enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    // Using the enum
    let move_direction = Direction::Up;

    match move_direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }

    // Enums can also have associated data, allowing you to store additional information with each variant.
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dice_game();
}
// This example demonstrates how to define an enum with multiple variants and use pattern matching to handle each variant.
// Enums can also have associated data, allowing you to store additional information with each variant.
// Here's an example of an enum with associated data:

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn dice_game() {
    let dice_roll = 9;

    match dice_roll {
        1..=6 => println!("You rolled a {}", dice_roll),
        _ => println!("Invalid roll!"),
    }
}
