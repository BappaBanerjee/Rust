# Enums and Pattern Matching

In this chapter, we’ll look at enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants. First we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.


## Defining an ENUM

-   Enums are a powerful feature in Rust that allow you to define a type that can be one of several different variants.
-   Each variant can have its own data and behavior, making enums a versatile tool for modeling complex data structures.
-   Enums can be used to represent a wide range of concepts, from simple states to complex data structures.

**Define an enum**

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Creating Instance of Enum

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## Enum with data

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

## In Enums each variant can have different types and amounts of associated data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

## Enum with wide variety

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Impl for Enums

There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```


## Options Enum

    -   The `Option` enum is a powerful feature in Rust that allows you to represent a value that may or may not be present.
    -   It is defined as `enum Option<T> { Some(T), None }`,
    -   where `T` is a type parameter that can be any type.
    -   The `Some` variant contains a value of type `T`, while the `None` variant represents the absence of a value.
    -   This is particularly useful for handling cases
    -   where a value may be optional or when dealing with operations that can fail.
    -   The `Option` enum is commonly used in Rust to handle situations where a value
    -   may not be available, such as when searching for an item in a collection or
    -   when a function may not return a value.
    -   It provides a safe way to handle the absence of a value without resorting to
    -   null pointers or other unsafe practices that can lead to runtime errors.
    -   The `Option` enum is a powerful tool for writing safe and expressive code in
    -   Rust, allowing you to handle optional values in a type-safe manner.
    -   It is often used in conjunction with pattern matching to handle the presence or absence of
    -   a value in a concise and readable way.



## Match control flow

Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things;


```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

**Having multiple lines inside match statement**

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

## Matching with Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Matches Are Exhaustive

There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities. 

Consider this version of our plus_one function, which has a bug and won’t compile:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn’t handle the None case, so this code will cause a bug. Luckily, it’s a bug Rust knows how to catch. If we try to compile this code, we’ll get error

```bash
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
```

## Catch-All Patterns and the _ Placeholder

Using enums, we can also take special actions for a few particular values, but for all other values take one default action. 

```rust
 let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

```

For the first two arms, the patterns are the literal values 3 and 7. For the last arm that covers every other possible value, the pattern is the variable we’ve chosen to name other. The code that runs for the other arm uses the variable by passing it to the move_player function.


**_ Placeholder**

Rust also has a pattern we can use when we want a ``catch-all`` but don’t want to use the value in the catch-all pattern: ``_`` is a ``special pattern`` that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

```rust

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```



## if..let

```rust
fn if_let() {
    // Example of if let
    let some_option = Some(5);
    
    if  let Some(value) = some_option {
        println!("The value is: {}", value);
    } else {
        println!("No value found");
    }
}  

```


```rust
fn let_else() {
    let some_option = Some(5);
    
    let Some(value) = some_option else {
        println!("No value found");
        return;
    };
    
    println!("The value is: {}", value);
}
```