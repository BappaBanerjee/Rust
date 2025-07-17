# Generic in Rust

## Defining a Generic function

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

## struct Defination

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

**struct with different types**

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

## In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

**We can also specify constraints on generic types when defining methods on the type.**

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is not of type f32 will not have this method defined.
