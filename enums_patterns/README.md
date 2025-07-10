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



