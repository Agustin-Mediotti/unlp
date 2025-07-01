use std::io;

fn main() {
    let boolean: bool = false;
    let mut input = String::new();

    println!("enter true or false: ");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");

    match input.trim() {
        "true" | "t" => {
            println!("{boolean} & true = {}", boolean & true);
            println!("{boolean} | true = {}", boolean | true);
        }
        "false" | "f" => {
            println!("{boolean} & false = {}", boolean & false);
            println!("{boolean} | false = {}", boolean | false);
        }
        _ => println!("{} is not a boolean", input.trim()),
    }
}
