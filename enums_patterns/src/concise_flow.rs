pub fn run() {
    if_let();
    let_else();
}

fn if_let() {
    // Example of if let
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("The value is: {}", value);
    } else {
        println!("No value found");
    }
}

fn let_else() {
    // Example of let else
    // This feature allows you to handle the case where the pattern does not match
    // in a more concise way compared to traditional match statements.
    // It is particularly useful when you want to extract a value from an Option or Result type
    // and handle the case where the value is not present without writing a full match statement.
    // This feature was introduced in Rust 1.70.
    // It allows you to use a more concise syntax for handling patterns that may not match.
    // The `let else` syntax is a way to handle the case where the pattern does not match,
    // allowing you to provide a fallback or alternative behavior in a more readable way.
    let some_option = Some(5);

    let Some(value) = some_option else {
        println!("No value found");
        return;
    };

    println!("The value is: {}", value);
}
