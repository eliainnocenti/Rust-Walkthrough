# Collections in Rust

Rust's standard library includes a number of very useful data structures called collections. The most common collections are vectors, strings, and hash maps.

## Vectors

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3, 4, 5];

    for i in &v2 {
        println!("{}", i);
    }
}
```

## Strings

Strings are implemented as a collection of bytes plus some methods to provide useful functionality when those bytes are interpreted as text.

```rust
fn main() {
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');

    let s2 = String::from("world");
    s.push_str(&s2);

    println!("{}", s);
}
```

## Hash Maps

Hash maps store a mapping of keys to values.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

Next: [Iterators and Closures](../07-Iterators-and-Closures/README.md)
