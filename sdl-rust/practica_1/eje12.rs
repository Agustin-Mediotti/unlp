fn main() {
    let t: (&str, Vec<u16>) = ("Powers of Two", vec![2, 4, 8, 16, 32, 64, 128]);
    println!("{}, {}", t.0, t.1.iter().sum::<u16>());
}
