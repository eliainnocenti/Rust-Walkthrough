# Basic Concepts in Rust

In this section, we'll cover the basic syntax and concepts in Rust.

## Variables and Mutability

In Rust, variables are immutable by default. You can make a variable mutable using the `mut` keyword.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
}
```

## Data Types

Rust is a statically typed language, which means that it must know the types of all variables at compile time. Here are some basic data types:

- Scalar types: integers, floating-point numbers, booleans, and characters.
- Compound types: tuples and arrays.

```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let t = true;
    let c = 'z';
    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    let arr = [1, 2, 3, 4, 5];
}
```

## Functions

Functions are defined using the `fn` keyword. Here's an example:

```rust
fn main() {
    println!("The value of five is: {}", five());
}

fn five() -> i32 {
    5
}
```

## Control Flow

Rust has the usual control flow constructs: `if`, `else`, `loop`, `while`, and `for`.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

Next: [Ownership and Borrowing](../03-Ownership-and-Borrowing/README.md)
