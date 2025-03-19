fn main() {
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');

    let s2 = String::from("world");
    s.push_str(&s2);

    println!("{}", s);
}
