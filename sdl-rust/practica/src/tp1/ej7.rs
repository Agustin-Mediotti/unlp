fn main() {
    const PI: f32 = 6.28;

    let arr = [2, 4, 8, 16, 32, 64]
        .iter()
        .map(|x| (*x as f32 * PI) as u32)
        .collect::<Vec<u32>>();

    println!("{:?}", arr);
}
