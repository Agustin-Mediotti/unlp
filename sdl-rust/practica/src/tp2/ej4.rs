pub fn cantidad_impares(list: &[i32]) -> i32 {
    list.iter().filter(|&&e| e % 2 != 0).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuenta_la_cantidad_de_impares_correctamente() {
        let enteros = [1, 2, 3, 5, 8, 11];
        let sin_impares = [0, 2, 4, 8, 12, 18];
        assert_eq!(
            cantidad_impares(&enteros),
            4,
            "Debería haber 4 impares en la lista [1, 2, 3, 5, 8, 11]"
        );
        assert_eq!(
            cantidad_impares(&sin_impares),
            0,
            "No hay impares en la lista, el resultado debería ser 0"
        );
        assert_eq!(
            cantidad_impares(&[]),
            0,
            "No hay elementos en la lista, el resultado debería ser 0"
        );
    }
}
