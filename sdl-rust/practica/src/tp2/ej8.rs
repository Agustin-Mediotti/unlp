#[allow(dead_code)]
pub fn sumar_arreglos(left: &[f32], right: &[f32]) -> Vec<f32> {
    left.iter().zip(right.iter()).map(|(x, y)| x + y).collect()
}
