pub fn sumar_arreglos(left: &[f32], right: &[f32]) -> Vec<f32> {
    left.iter().zip(right.iter()).map(|(x, y)| x + y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_elemento_a_elemento_dos_arreglos() {
        let a = [2.0, 4.2, 8.4, 1.1];
        let b = [2.0, 4.2, 8.4, 1.1];
        assert_eq!(
            sumar_arreglos(&a, &b),
            [4.0, 8.4, 16.8, 2.2],
            "La suma de los arreglos no coincide con lo esperado"
        );
    }
}
