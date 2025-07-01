fn main() {
    let arr_01: Vec<u32> = vec![2, 4, 8, 16, 32];
    let arr_02: Vec<u32> = vec![64, 128, 256, 512, 1024];
    let arr_03: Vec<u32> = arr_01
        .iter()
        .zip(arr_02.iter())
        .map(|(a, b)| a + b)
        .collect();
    println!("{:?}", arr_03);
}
