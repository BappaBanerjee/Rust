pub fn run() {
    /*
    *   Variables can be mutable or immutable.
    *   Immutable variables cannot be changed after they are set.
    *   Mutable variables can be changed.
    *   By default, variables are immutable.
     */
    // let x = 5; // Immutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    
    // Constants
    /*
    *   Constants are always immutable.
    *   They must have a type annotation.
    *   Constants can be set in any scope, including inside functions.
    *   Constants can only be set to a constant expression, not a variable.
    *   Constants are declared using the `const` keyword.
    *   Constants are not allowed to be mutable.
    *   Constants can be used in any scope, including inside functions.
    *   Constants are not allowed to be set to a variable.
    *   Constants are not allowed to be set to a function call.
    *   Constants are not allowed to be set to a value that is not a constant expression.
     */
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {MAX_POINTS}");

    // Shadowing
    /*  Shadowing allows you to change the value of a variable without making it mutable.
    *   Shadowing allows you to change the type of a variable.
    */
    let y = 5;
    println!("The value of y is: {y}");
    let y = y + 1; // Shadowing the variable y
    println!("The value of y is: {y}");
    let y = "Hello"; // Changing the type of the variable y
    println!("The value of y is: {y}");
}