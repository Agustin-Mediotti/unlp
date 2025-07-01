pub fn duplicar_valores(lista: &[f32]) -> Vec<f32> {
    lista.iter().map(|&e| e * 2.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplica_los_valores_correctamente() {
        let lista = [1.0, 2.2, 4.8, 5.0, 0.0];
        assert_eq!(
            duplicar_valores(&lista),
            [2.0, 4.4, 9.6, 10.0, 0.0],
            "Los valores duplicados no coinciden con lo esperado"
        );
    }
}
