use std::fmt::Display;
struct Point<T> {
    x: T,
    y: T,
}

pub fn generic_example() {
    // Defining a generic function

    // Using the generic function
    print_generic(42);
    print_generic("Hello, Rust!");

    //struct with generic type
    let point = Point { x: 5, y: 10 };
    println!("Point: ({}, {})", point.x, point.y);

    //enum with generic type
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let success: Result<i32, &str> = Result::Ok(200);
    let error: Result<i32, &str> = Result::Err("An error occurred");
    match success {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(e) => println!("Error: {}", e),
    }
    match error {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(e) => println!("Error: {}", e),
    }
}

fn print_generic<T: Display>(item: T) {
    println!("Item: {}", item);
}
