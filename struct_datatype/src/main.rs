#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    // This is an associated function, not a method
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    struct_example();

    let rectangle = Rectangle {
        width: 5,
        length: 10,
    };

    // Using the function to calculate area
    let area = calculate_area(&rectangle);

    println!("area is {area}");

    // Using the method defined in the impl block
    let area_method = rectangle.area();
    println!("area using method is {area_method}");
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn struct_example() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);

    //printing the debug
    println!("user1 is {user1:#?}");

    // println!("user1 is {user1:?}");
    //tuple struct
    /*
     *   Tuple structs are a type of struct that uses a tuple to group multiple values together.
     *   They are defined using the `struct` keyword followed by the name of the struct
     *   and a tuple of values.
     *   Tuple structs can be used to create a new type that is distinct from other types
     *   even if the tuple has the same types as another tuple struct.
     *   They are useful for creating simple data structures that do not require named fields.
     *  Example:
     */
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
}
