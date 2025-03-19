# Advanced Topics in Rust

In this section, we'll cover some advanced topics and best practices in Rust.

## Lifetimes

Lifetimes are a way of describing the scope of references.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

You can find the code for this section in the [workspace directory](workspace/lifetimes.rs).

## Traits

Traits are a way to define shared behavior in Rust.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}
```

You can find the code for this section in the [workspace directory](workspace/traits.rs).

## Macros

Macros are a way of writing code that writes other code.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

You can find the code for this section in the [workspace directory](workspace/macros.rs).

## Unsafe Rust

Unsafe Rust gives you more control over low-level details but requires you to ensure safety.

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```

You can find the code for this section in the [workspace directory](workspace/unsafe_rust.rs).

Next: [Back to Introduction](../00-Introduction/README.md)
