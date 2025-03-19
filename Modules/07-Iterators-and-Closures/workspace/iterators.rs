fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for val in v_iter {
        println!("{}", val);
    }
}
