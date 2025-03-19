fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let t = true;
    let c = 'z';
    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    let arr = [1, 2, 3, 4, 5];
}
