/*
Write a Rust program to count how many vowels (a, e, i, o, u) are present in a given string.
The check should be case-insensitive.
*/

fn main() {
    let str = String::from("Let's get rusty!");
    let mut count = 0;

    let vowel = ['a', 'e', 'i', 'o', 'u'];

    // basic way
    for char in str.chars() {
        if vowel.contains(&char.to_ascii_lowercase()) {
            count += 1;
        }
    }

    println!("count using loop : {count}");

    //rusty approach
    let vowels: Vec<char> = str
        .chars()
        .filter(|x| vowel.contains(&x.to_ascii_lowercase()))
        .collect();
    let count = str
        .chars()
        .filter(|x| vowel.contains(&x.to_ascii_lowercase()))
        .count();

    println!("count using filter : {count}");
    println!("{vowels:?}");
}
