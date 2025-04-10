#[allow(dead_code)]
pub fn cantidad_en_rango(lista: &[i32], inferior: &i32, superior: &i32) -> u32 {
    lista
        .iter()
        .filter(|x| x >= &inferior && x <= &superior)
        .count() as u32
}
