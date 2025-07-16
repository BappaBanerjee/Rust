# HashMap in Rust

## Creating a New Hash Map

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

## Accessing Values in a Hash Map

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key.

## HashMap Iteration

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

## Hash Maps and Ownership

For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values.

## Updating a Hash Map

### Overwriting a Value

If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
```

### Adding a Key and Value Only If a Key Isn’t Present

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
```

The ``or_insert`` method on ``Entry`` is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value. 

### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
```

This code will print {"world": 2, "hello": 1, "wonderful": 1}.

The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text. The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.