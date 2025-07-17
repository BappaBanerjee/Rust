fn main() {
    println!("Vector");

    vector_example();
    string_example();
    hashmap_example();
}

fn hashmap_example() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Score for {}: {}", team_name, s),
        None => println!("No score found for {}", team_name),
    }

    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //adding a new key-value pair if not exists
    scores.entry(String::from("Green")).or_insert(30);
}

fn string_example() {
    //creating a new empty string
    let mut s = String::new();

    //defining a string
    let data = "initial contents";

    //Using the to_string method to create a String from a string literal
    let s = data.to_string();

    //Using the String::from function to create a String from a string literal
    let s = String::from("Hello world");

    //Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");

    // Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');

    //Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //The format! macro works like println!, but instead of printing the output to the screen,
    //it returns a String with the contents.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    /* NOTE */
    // Indexing into Strings in rust is not allowed because string are stored in Unicode scalar value
    //which take 2 bytes of space for each characters
}

fn vector_example() {
    //defining a vector
    let vec1: Vec<i32> = Vec::new();

    //defining and initializing
    let mut vec2 = vec![10, 20, 30];

    //pushing data in vector
    vec2.push(40);
    vec2.push(50);
    vec2.push(60);

    //reading an element from vector using get
    let third = vec2.get(2);
    match third {
        Some(third) => {
            println!("gotch {third}")
        }
        None => {
            println!("oops")
        }
    }

    //using index
    let third: &i32 = &vec2[2];
    println!("The third element is {third}");

    //vector iteration : immutable
    for i in &vec2 {
        println!("element is {i}");
    }

    //mutable vector change
    for i in &mut vec2 {
        *i += 50;
    }

    println!("{vec2:?}");

    //using enum with vector
    #[derive(Debug)]
    enum SphreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let cell: Vec<SphreadSheetCell> = vec![
        SphreadSheetCell::Int(3),
        SphreadSheetCell::Float(54.50),
        SphreadSheetCell::Text(String::from("Rusty")),
    ];

    println!("{cell:?}");
}
