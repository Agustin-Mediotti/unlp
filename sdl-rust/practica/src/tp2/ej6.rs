#[allow(dead_code)]
pub fn longitud_de_cadenas(lista: &[&str]) -> Vec<u32> {
    lista.iter().map(|e| e.chars().count() as u32).collect()
}
