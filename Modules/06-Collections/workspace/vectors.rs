fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3, 4, 5];

    for i in &v2 {
        println!("{}", i);
    }
}
