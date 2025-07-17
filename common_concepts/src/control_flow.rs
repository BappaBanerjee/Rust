/*
*   Unlike languages such as Ruby and JavaScript,
*   Rust will not automatically try to convert non-Boolean types to a Boolean.
*   You must be explicit and always provide if with a Boolean as its condition.
*/

pub fn run() {
    // Control Flow
    /*
     *   Control flow allows you to change the order of execution of your code.
     *   Rust has several control flow constructs, including if statements, loops, and match expressions.
     *   Control flow constructs allow you to make decisions in your code and repeat actions.
     *   Control flow constructs can be nested to create complex logic.
     */

    if_loop();
    loop_example();
    while_loop_example();
    for_loop_example();
    match_example();
    // Note: The `if` statement is not a loop, but it is included here
    // because it is a control flow construct that allows you to make decisions in your code.
    /*
     *   The `if` statement allows you to execute code conditionally based on a Boolean expression.
     *   The `loop` construct allows you to create an infinite loop that can be exited with a `break` statement.
     *   The `while` loop allows you to repeat code while a condition is true.
     *   The `for` loop allows you to iterate over a collection or a range of values.
     *   The `match` expression allows you to match a value against multiple patterns and execute code based on the match.
     */
}

fn if_loop() {
    // If statement
    let number = 5;
    if number < 10 {
        println!("The number is less than 10");
    } else {
        println!("The number is 10 or greater");
    }
    // If-else statement with multiple conditions
    if number < 5 {
        println!("The number is less than 5");
    } else if number < 10 {
        println!("The number is between 5 and 10");
    } else {
        println!("The number is 10 or greater");
    }

    // If statement with a condition that evaluates to a Boolean expression
    let is_even = number % 2 == 0;
    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn loop_example() {
    // Loop
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count; // Exit the loop when count reaches 5
        }
        // println!("Count is: {}", count);
    };

    println!("Loop exited with result: {}", result);

    // Loop with a label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop_example() {
    // While loop
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1; // Increment count
    }
}

fn for_loop_example() {
    // For loop
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number is: {}", number);
    }

    // For loop with a range
    for i in 0..5 {
        println!("i is: {}", i);
    }
}

fn match_example() {
    // Match expression
    let value = 2;
    match value {
        1 => println!("Value is one"),
        2 => println!("Value is two"),
        _ => println!("Value is something else"),
    }

    // Match with multiple patterns
    let another_value = 3;
    match another_value {
        1 | 2 => println!("Value is one or two"),
        3 => println!("Value is three"),
        _ => println!("Value is something else"),
    }
}
