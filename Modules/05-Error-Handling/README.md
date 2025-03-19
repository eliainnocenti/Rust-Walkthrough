# Error Handling in Rust

Rust has a strong emphasis on error handling. It uses the `Result` and `Option` types to handle recoverable and unrecoverable errors.

## The `Result` Type

The `Result` type is used for functions that can return an error.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
        },
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

## The `Option` Type

The `Option` type is used when a value could be something or nothing.

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
```

## `unwrap` and `expect`

You can use `unwrap` and `expect` to handle errors in a more concise way.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

Next: [Collections](../06-Collections/README.md)
