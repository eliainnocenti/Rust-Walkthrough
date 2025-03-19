# Structs and Enums in Rust

Structs and enums are used to create custom data types in Rust.

## Structs

A struct is a custom data type that lets you name and package together multiple related values.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

## Enums

Enums allow you to define a type by enumerating its possible variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Write(String::from("hello"));
}
```

## Pattern Matching

Pattern matching is used to work with enums.

```rust
fn main() {
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}
```

Next: [Error Handling](../05-Error-Handling/README.md)
