# Struct in RUST

Structs are similar to tuples. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

## Defining and initiating structs

**Define a struct**

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

**Initiating a struct**

To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields.

```rust
    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    }
```

### To get the specific value of the struct we use

```rust
println!("Username: {}", user1.username);
```


### change the value in the email field of a mutable User instance.

```rust 
    user1.email = String::from("anotheremail@example.com");
```

## Using the Field Init Shorthand
Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite 

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## Creating Instances from Other Instances with Struct Update Syntax

It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using struct update syntax.

***Example without update syntax***
```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

***Example with update syntax***

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

The code creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1. The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

**💡 NOTE:**  
The **struct update syntax** uses `=` like assignment because it **moves** the data, similar to the behavior discussed in the **"Variables and Data Interacting with Move"** section.

In this example, we can no longer use `user1` after creating `user2` because the `String` in the `username` field of `user1` was **moved** into `user2`.

However, if we gave `user2` **new `String` values** for both `email` and `username`, and only reused `active` and `sign_in_count` from `user1`, then `user1` would remain valid after creating `user2`.

This is because `active` and `sign_in_count` are **Copy** types (simple stack-only data), so they behave as discussed in the **"Stack-Only Data: Copy"** section.

✅ You can still use fields from `user1` that weren't moved:
```rust
println!("{}", user1.email);
```


## Tuple struct

    -  Tuple structs are a type of struct that uses a tuple to group multiple values together.
    -   They are defined using the `struct` keyword followed by the name of the struct
    -   and a tuple of values.
    -   Tuple structs can be used to create a new type that is distinct from other types
    -   even if the tuple has the same types as another tuple struct.
    -   They are useful for creating simple data structures that do not require named fields.

```rust

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

```

## Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Methods

Methods are similar to functions. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.

To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

```

-The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

- The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.


## Methods with More Parameters

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Multiple impl Blocks

Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.

```rust 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```


# Summary 

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.



