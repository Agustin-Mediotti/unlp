#[allow(dead_code)]
pub fn cantidad_de_cadenas_mayor_a(list: &[&str], limite: u32) -> u32 {
    list.iter()
        .filter(|f| f.chars().count() as u32 > limite)
        .count() as u32
}
