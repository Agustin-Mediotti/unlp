use std::io;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let pi: f32 = 3.14;

    println!("enter a number: ");

    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line!");
    let num: u32 = buf.trim().parse().expect("not an integer!");

    println!("{num} * {pi} = {}", num as f32 * pi);
    println!("{num} / {pi} = {}", num as f32 / pi);
    println!("{num} + {pi} = {}", num as f32 + pi);
    println!("{num} - {pi} = {}", num as f32 - pi);

    Ok(())
}
