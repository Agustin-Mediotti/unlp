use std::io;

fn main() {
    let text_vec = vec![
        "Hello World",
        "UNLP Informatica",
        "Seminario 2025",
        "Rust is awesome",
        "Lateralus",
    ];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");

    if text_vec.contains(&input.trim()) {
        println!("I know that one");
    } else {
        println!("A new spell?");
    }
}
