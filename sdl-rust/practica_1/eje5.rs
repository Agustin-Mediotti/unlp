use std::io;

fn main() {
    let text = String::from("I'm not screaming, ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("bad input");

    println!("{}", (text + &input).to_uppercase());
}
