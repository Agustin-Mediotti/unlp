#[allow(dead_code)]
pub fn cantidad_impares(list: &[i32]) -> i32 {
    list.iter().filter(|&&e| e % 2 != 0).count() as i32
}
