use std::io;

fn main() {
    const text: &str = "Seminario de Lenguajes 2025: Rust";

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    let input: char = input.chars().collect::<Vec<_>>()[0];

    let t = text.chars().filter(|c| *c == input).count();

    println!("{t}");
}
