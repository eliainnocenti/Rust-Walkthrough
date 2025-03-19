# Modules and Packages in Rust

Modules and packages help you organize your code and manage dependencies.

## Modules

Modules let you organize your code into groups. You can define modules using the `mod` keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

## Packages and Crates

A package is a bundle of one or more crates. A crate is a binary or library.

### Creating a Library Crate

1. **Create a New Library**: Use Cargo to create a new library:

    ```sh
    cargo new my_library --lib
    cd my_library
    ```

    This creates a new directory named `my_library` with the following structure:

    ```
    my_library/
    ├── Cargo.toml
    └── src/
        └── lib.rs
    ```

2. **Writing Code**: Open `src/lib.rs` and add the following code:

    ```rust
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
    ```

3. **Using the Library**: You can use this library in another project by adding it as a dependency in `Cargo.toml`.

    ```toml
    [dependencies]
    my_library = { path = "../my_library" }
    ```

    Then, you can use the library in your code:

    ```rust
    use my_library::add;

    fn main() {
        let result = add(2, 3);
        println!("The result is {}", result);
    }
    ```

Next: [Concurrency](../09-Concurrency/README.md)
