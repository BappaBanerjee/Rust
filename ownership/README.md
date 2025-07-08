# 📚 Rust Ownership & Memory — Quick Guide

This guide summarizes Rust’s **ownership** model and memory management concepts for fast recall.

---

## 📌 Stack vs Heap:
- **Stack**
  - Stores simple, fixed-size data (integers, chars, etc.)
  - Fast, auto-managed (push/pop).
  - Example:  
    ```rust
    let x = 5;  // Stored on stack
    ```
  
- **Heap**
  - Stores dynamically-sized data (like `String`).
  - Slower, needs manual allocation & deallocation (handled by Rust safely).
  - Example:
    ```rust
    let s = String::from("hello");  // Stored on heap
    ```

---

## 📌 String Literals (`&str`) vs `String`
- **String Literals (`&str`):**
  - Fixed-size, immutable.
  - Stored in binary (stack).
  - Example:
    ```rust
    let s = "hello";  // &str
    ```

- **`String` Type:**
  - Growable, heap-allocated.
  - Mutable and commonly used for dynamic strings.
  - Example:
    ```rust
    let mut s = String::from("hello");
    s.push_str(", world!");
    ```

---

## 📌 Ownership Rules:
1. Each value has exactly **one owner**.
2. Ownership is **transferred** (moved) when assigned or passed to functions.
3. When the owner goes out of scope, the value is automatically dropped.

---

## 📌 Ownership Example:
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moves to s2 (s1 becomes invalid)
```


## Move

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
```

You’ll get an error like this because Rust prevents you from using the invalidated reference
To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. 

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.


## Scope and Assignment

```rust
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world!");
```

We initially declare a variable s and bind it to a String with the value "hello". Then we immediately create a new String with the value "ahoy" and assign it to s. At this point, nothing is referring to the original value on the heap at all.

The original string thus immediately goes out of scope. Rust will run the drop function on it and its memory will be freed right away. When we print the value at the end, it will be "ahoy, world!".


## copy trait

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

## The Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
