fn main() {
    //ownership
    let s1 = String::from("Hello, world!");
    let s2 = s1; // Move ownership from s1 to s2    
    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid
    println!("{}", s2); // This is valid, as s2 now owns the String
    // Ownership and functions
    let s3 = String::from("Hello, Rust!");
    takes_ownership(s3); // s3's ownership is moved to the function
    // println!("{}", s3); // This line would cause a compile-time error because s3
    // is no longer valid


    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

    let s4 = String::from("Hello, again!");
    let s5 = takes_and_gives_back(s4); // Ownership is moved and then
    // returned
    println!("{}", s5); // This is valid, as s5 now owns the String



    // Ownership and references
    let s6 = String::from("Hello, references!");
    let len = calculate_length(&s6); // Pass a reference    
    println!("The length of '{}' is {}.", s6, len); // s6 is still valid here
    
    
    // Mutable references
    /*
    *   Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 
    */
    let mut s7 = String::from("Hello, mutable references!");
    change(&mut s7); // Pass a mutable reference
    println!("{}", s7); // s7 is modified by the function



    // Multiple mutable references
    let mut s8 = String::from("Hello, multiple mutable references!");
    {
        let r1 = &mut s8; // Create a mutable reference
        r1.push_str(" Modified by r1.");
    } // r1 goes out of scope here, so we can create another mutable reference
    {
        let r2 = &mut s8; // Create another mutable reference
        r2.push_str(" Modified by r2.");
    } // r2 goes out of scope here, so s8 can be used again
    println!("{}", s8); // s8 is modified by both r1 and r2



    // Ownership and slices
    let s9 = String::from("Hello, slices!");
    let slice = &s9[0..5]; // Create a slice of the String
    println!("The slice is: {}", slice); // s9 is still valid here
    

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}