fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    let v3: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", v3);
}
