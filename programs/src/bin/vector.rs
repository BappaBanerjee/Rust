// Write a Rust program that takes a vector of integers and removes all even numbers, returning only the odd ones.

fn main() {
    let mut vec = vec![1,2,3,4,5,9,13, 18];

    //solution 1
    let mut vec2 : Vec<i32> = Vec::new();
    for i in &vec {
        if i % 2 == 0 {
            vec2.push(*i);
        }
    }

    println!("{vec2:?}");

    //solution 2
    vec.retain(|&x| x %2 == 0);
    println!("{vec:?}");

    //solution 3
    let vec2 : Vec<i32> = vec.into_iter().filter(|x| x%2 == 0).collect();
    println!("{vec2:?}");
}