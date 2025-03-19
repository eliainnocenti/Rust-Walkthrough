# Introduction to Rust

Welcome to the Rust programming language! Rust is a systems programming language that focuses on safety, speed, and concurrency. It is designed to help you write reliable and efficient software.

## Setting Up the Environment

1. **Install Rust**: The easiest way to install Rust is by using `rustup`, the Rust toolchain installer. Run the following command in your terminal:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Verify Installation**: After installation, you can verify that Rust is installed correctly by running:

    ```sh
    rustc --version
    ```

3. **Install a Code Editor**: While you can use any text editor, it's recommended to use an editor with Rust support. Some popular choices are:
    - Visual Studio Code with the Rust extension
    - IntelliJ IDEA with the Rust plugin
    - Sublime Text with Rust support

4. **Hello, World!**: Create a new directory for your first Rust project and navigate into it:

    ```sh
    mkdir hello_world
    cd hello_world
    ```

    Create a new file named `main.rs` and add the following code:

    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

    Compile and run the program:

    ```sh
    rustc main.rs
    ./main
    ```

Congratulations! You've written and run your first Rust program.

Next: [Getting Started](../01-Getting-Started/README.md)
