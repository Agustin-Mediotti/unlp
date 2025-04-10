#[allow(dead_code)]
pub fn suma_pares(list: &[i32]) -> i32 {
    list.iter().filter(|&&n| n % 2 == 0).sum()
}
