pub fn suma_pares(list: &[i32]) -> i32 {
    list.iter().filter(|&&n| n % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_correctamente_la_suma_de_pares() {
        let enteros_pares = [2, 4, 8, 16];
        let enteros_impares = [1, 3, 5, 11];
        assert_eq!(
            suma_pares(&enteros_pares),
            30,
            "La suma de los pares [2, 4, 8, 16] debería ser 30"
        );
        assert_eq!(
            suma_pares(&enteros_impares),
            0,
            "No hay pares, el resultado debería ser 0"
        );
    }
}
