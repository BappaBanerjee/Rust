pub fn run() {
    let x : u8 = 255; // This will cause a compile-time error because 256 is out of range for u8
    println!("The value of x is: {}", x);

    //Data Types
    /*
    *   Rust is a statically typed language, meaning that the type of a variable must be known at compile time.
    *   Rust has a strong type system, meaning that the type of a variable cannot be
    *   changed after it is set.
    *   Rust has a rich set of data types, including integers, floating-point numbers,
    *   booleans, characters, and strings.
    *   Rust also has compound data types, such as tuples and arrays.
    *   Rust has a powerful type inference system, meaning that the type of a variable can
    *   often be inferred by the compiler without needing to explicitly specify it.
    */

    //Scalar Types
    /*  
    *   Scalar types represent a single value.
    *   Rust has four primary scalar types: 
    *   integers, floating-point numbers, booleans, and characters.
    *   Integers can be signed or unsigned, and can have different sizes
    *   (e.g., i8, i16, i32, i64, i128, u8, u16, u32, u64, u128).
    *   Floating-point numbers can be either f32 or f64.
    *   Booleans can be either true or false.
    *   Characters represent a single Unicode scalar value and are denoted by single quotes (e.g., 'a', '1', '😊').
    *   Rust also has a unit type (), 
    *   which represents an empty value and is used in functions that do not return a value.
    *   The unit type is often used as a placeholder in function signatures or when a function
    *   does not need to return a value.
    *   The unit type is also used in expressions that do not produce a value, such
    *   as when a function is called for its side effects.
    *   The unit type is often used in Rust to indicate that a function does not return
    *   a value, or that a value is not needed.
    *   The unit type is represented by the empty tuple ().
     */

    let a: i32 = 10; // Signed integer
    let b: u32 = 20; // Unsigned integer
    let c: f64 = 3.14; // Floating-point number
    let d: bool = true; // Boolean value
    let e: char = 'A'; // Character
    let f: () = (); // Unit type

    println!("Integer: {}, Unsigned Integer: {}, Floating-point: {}, Boolean: {}, Character: {}, Unit type: {:?}", a, b, c, d, e, f);


    //Compound Types
    /*
    *   Compound types can group multiple values into one type.
    *   Rust has two primary compound types: tuples and arrays.
    *   Tuples can hold values of different types and are defined using parentheses (e.g., (1, "hello", true)).
    *   Arrays hold values of the same type and are defined using square brackets (e.g., [1, 2, 3, 4, 5]).
    *   Tuples can have a fixed size and can hold values of different types.
    *   Arrays can have a fixed size and can hold values of the same type.
    *   Tuples are useful for grouping related values together, while arrays are useful for storing
    *   collections of values of the same type.
     */

    let tuple: (i32, f64, char) = (42, 3.14, 'x'); // Tuple with different types
    println!("Tuple: {:?}", tuple); 
    // Accessing elements in a tuple
    let (x, y, z) = tuple; // Destructuring the tuple
    println!("Tuple elements: x = {}, y = {}, z = {}", x, y, z);

    // Accessing elements by index
    // Note: Tuples are fixed-size and can hold different types, so you must know
    // the types of the elements when accessing them by index.
    // This is not recommended for readability, but it is possible.
    // Accessing elements by index is not idiomatic in Rust, but it is possible.
    // It is more common to destructure the tuple as shown above.
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tuple elements: five_hundred = {}, six_point_four = {}, one = {}", five_hundred, six_point_four, one);

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array of integers with fixed size 
    println!("Array: {:?}", array); // Printing the array
    // Accessing elements in an array
    let first_element = array[0]; // Accessing the first element of the array
    let second_element = array[1]; // Accessing the second element of the array
    println!("First element: {}, Second element: {}", first_element, second_element);

    let a = [3; 5]; // An array of 5 elements, all initialized to 3
    println!("Array with all elements initialized to 3: {:?}", a);

    // let check = array[10]; // This will cause a runtime error if uncommented, as it tries to access an out-of-bounds index
    // println!("This will cause a runtime error: {}", check); // Uncommenting this line
    // will cause a panic at runtime because the index is out of bounds.
    
}