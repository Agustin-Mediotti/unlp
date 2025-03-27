use std::io;

fn main() {
    let a: u32 = 30;
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let input: u32 = input.trim().parse().unwrap();

    println!("({a} + {input})^2 = {}", (a + input) * 2);
}
