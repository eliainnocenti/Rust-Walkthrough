# Getting Started with Rust

Now that you have Rust installed, let's dive into writing your first Rust program using Cargo, Rust's package manager and build system.

## Creating a New Project

1. **Create a New Project**: Use Cargo to create a new project:

    ```sh
    cargo new hello_cargo
    cd hello_cargo
    ```

    This creates a new directory named `hello_cargo` with the following structure:

    ```
    hello_cargo/
    ├── Cargo.toml
    └── src/
        └── main.rs
    ```

2. **Understanding the Structure**:
    - `Cargo.toml`: The configuration file for your project.
    - `src/main.rs`: The main source file for your project.

3. **Writing Code**: Open `src/main.rs` and you should see the following code:

    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

4. **Building and Running**: Build and run your project using Cargo:

    ```sh
    cargo build
    cargo run
    ```

    You should see `Hello, world!` printed to the console.

You can find the code for this section in the [workspace directory](workspace/hello_cargo.rs).

## Exploring Cargo

Cargo makes it easy to manage dependencies and build your project. Here are some useful commands:

- `cargo check`: Quickly checks your code to make sure it compiles without producing an executable.
- `cargo test`: Runs tests for your project.
- `cargo doc --open`: Builds documentation for your project and opens it in your browser.

Next: [Basic Concepts](../02-Basic-Concepts/README.md)
