# Ownership and Borrowing in Rust

Rust's ownership model is one of its most unique and powerful features. It ensures memory safety without needing a garbage collector.

## Ownership

Every value in Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause a compile-time error
}
```

## Borrowing

Instead of transferring ownership, you can borrow a value using references. References are immutable by default.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Mutable References

You can also borrow a value mutably, but you can only have one mutable reference to a particular piece of data in a particular scope.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}
```

Next: [Structs and Enums](../04-Structs-and-Enums/README.md)
