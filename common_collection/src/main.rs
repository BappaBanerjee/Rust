fn main() {
    println!("Vector");

    vector_example();
}

fn vector_example() {
    //defining a vector
    let vec1 : Vec<i32> = Vec::new();

    //defining and initializing
    let mut vec2 = vec![10,20,30];

    //pushing data in vector
    vec2.push(40);
    vec2.push(50);
    vec2.push(60);

    //reading an element from vector using get
    let third = vec2.get(2);
    match third {
        Some(third) => {
            println!("gotch {third}")
        },
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
    for i in &mut vec2{
            *i += 50;
    }

    println!("{vec2:?}");

    //using enum with vector
    #[derive(Debug)]
    enum SphreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let cell : Vec<SphreadSheetCell> = vec![
        SphreadSheetCell::Int(3),
        SphreadSheetCell::Float(54.50),
        SphreadSheetCell::Text(String::from("Rusty"))
    ];

    println!("{cell:?}");
}
