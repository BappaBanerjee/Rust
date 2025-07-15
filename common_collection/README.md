# Common Collections

In this chapter, we’ll discuss three collections that are used very often in Rust programs:

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

## Vectors

## Creating a New Vector

```rust
 let v: Vec<i32> = Vec::new();
 ```

 ``Note`` that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store. This is an important point.

 Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it. Listing 8-2 creates a new Vec<i32> that holds the values 1, 2, and 3. The integer type is i32 because that’s the default integer type,

 ```rust
  let v = vec![1, 2, 3];
```

## Updating a Vector

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

## Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or by using the get method.

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

When we run this code, the first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Your code will then have logic to handle having either ``Some(&element) `` or ``None``.

## Borrowing and refernce

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```

The code might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way ``vectors work``: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to ``deallocated memory``. The borrowing rules prevent programs from ending up in that situation.

## Iterating Over the Values in a Vector

**use a for loop to get immutable references to each element in a vector of i32 values and print them.**

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

**Iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.**

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

To change the value that the mutable reference refers to, we have to use the ``* dereference operator`` to get to the value in i before we can use the += operator. 

## Using an Enum to Store Multiple Types

Vectors can only store values that are of the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled