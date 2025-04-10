#[allow(dead_code)]
pub fn es_primo(num: u64) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        _ => !(2..=((num as f64).sqrt() as u64)).any(|i| num % i == 0),
    }
}
