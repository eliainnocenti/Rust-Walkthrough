# Iterators and Closures in Rust

Iterators and closures are powerful features in Rust that allow you to work with sequences of data and anonymous functions.

## Iterators

An iterator is a trait that allows you to iterate over a sequence of elements.

```rust
fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for val in v_iter {
        println!("{}", val);
    }
}
```

You can find the code for this section in the [workspace directory](workspace/iterators.rs).

## Closures

Closures are anonymous functions that you can save in a variable or pass as arguments to other functions.

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one(5));

    let add_two = |x| x + 2;
    println!("{}", add_two(5));
}
```

You can find the code for this section in the [workspace directory](workspace/closures.rs).

## Using Closures with Iterators

You can use closures with iterator methods like `map` and `filter`.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    let v3: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", v3);
}
```

You can find the code for this section in the [workspace directory](workspace/using_closures_with_iterators.rs).

Next: [Modules and Packages](../08-Modules-and-Packages/README.md)
