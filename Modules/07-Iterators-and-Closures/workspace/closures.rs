fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one(5));

    let add_two = |x| x + 2;
    println!("{}", add_two(5));
}
