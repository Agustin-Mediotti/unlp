#[allow(dead_code)]
pub fn cantidad_de_mayores(lista: &[u32], limite: &u32) -> u32 {
    lista.iter().filter(|e| e > &limite).count() as u32
}
