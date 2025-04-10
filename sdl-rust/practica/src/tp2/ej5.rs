#[allow(dead_code)]
pub fn duplicar_valores(lista: &[f32]) -> Vec<f32> {
    lista.iter().map(|&e| e * 2.0).collect()
}
