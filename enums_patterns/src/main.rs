mod concise_flow;
mod match_flow;

fn main() {
    println!("Enums and Patterns in Rust!");
    enums_patters();
    match_flow::match_flow();
    concise_flow::run();
}

fn enums_patters() {
    //enums
    /*
     *   Enums are a powerful feature in Rust that allow you to define a type that can be one of several different variants.
     *   Each variant can have its own data and behavior, making enums a versatile tool for modeling complex data structures.
     *   Enums can be used to represent a wide range of concepts, from simple states to complex data structures.
     *   They can also be used in pattern matching, allowing you to easily handle different cases based on the variant of the enum.
     *   Enums can be defined with or without associated data, and they can also implement methods and traits.
     *   Enums are often used in conjunction with pattern matching to handle different cases based on the variant of the enum.
     *   This allows you to write concise and expressive code that is easy to read and maintain.
     *   Enums can also be used to define custom error types
     *   and can be combined with other Rust features like structs and traits to create powerful abstractions.
     *   Overall, enums are a fundamental part of Rust's type system and provide a
     *   flexible way to model complex data and behavior in your programs.
     *   Enums can be used to represent a wide range of concepts, from simple states
     *   to complex data structures. They can also be used in pattern matching, allowing you
     *   to easily handle different cases based on the variant of the enum.
     *   Enums can be defined with or without associated data, and they can also implement
     *   methods and traits. Enums are often used in conjunction with pattern matching to handle
     *   different cases based on the variant of the enum. This allows you to write concise
     *   and expressive code that is easy to read and maintain.
     */

    //enum with param
    enum IPKind {
        V4(String),
        V6(String),
    };

    //enum with different params
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //enum with variety of types
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    //Enums have impl
    impl Message {
        fn call(&self) {
            // method implementation
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                Message::Write(text) => println!("Write: {}", text),
                Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
            }
        }
    }
    // Using the enum
    let msg = Message::Write(String::from("Hello, Rust!"));
    msg.call();

    // option enum
    /*
     *   The `Option` enum is a powerful feature in Rust that allows you to represent a value that may or may not be present.
     *   It is defined as `enum Option<T> { Some(T), None }`,
     *   where `T` is a type parameter that can be any type.
     *   The `Some` variant contains a value of type `T`, while the `None` variant represents the absence of a value.
     *   This is particularly useful for handling cases
     *   where a value may be optional or when dealing with operations that can fail.
     *   The `Option` enum is commonly used in Rust to handle situations where a value
     *   may not be available, such as when searching for an item in a collection or
     *   when a function may not return a value.
     *   It provides a safe way to handle the absence of a value without resorting to
     *   null pointers or other unsafe practices that can lead to runtime errors.
     *   The `Option` enum is a powerful tool for writing safe and expressive code in
     *   Rust, allowing you to handle optional values in a type-safe manner.
     *   It is often used in conjunction with pattern matching to handle the presence or absence of
     *   a value in a concise and readable way.
     */

    enum Option<T> {
        None,
        Some(T),
    }
    // Example usage of Option enum
    fn find_item<T: PartialEq>(items: &[T], item: T) -> Option<usize> {
        for (index, current_item) in items.iter().enumerate() {
            if *current_item == item {
                return Option::Some(index);
            }
        }
        Option::None
    }
    // Example usage of find_item function
    let items = vec![1, 2, 3, 4, 5];
    let item_to_find = 3;
    match find_item(&items, item_to_find) {
        Option::Some(index) => println!("Item found at index: {}", index),
        Option::None => println!("Item not found"),
    }
}
