pub fn cantidad_de_cadenas_mayor_a(list: &[&str], limite: u32) -> u32 {
    list.iter()
        .filter(|f| f.chars().count() as u32 > limite)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuenta_cadenas_con_mas_caracteres_que_el_limite() {
        let lista = [
            "String Corto",
            "Cadena de string medianamente larga",
            "Cadena de string relativamente mas largo",
            "Una cadena de string considerablemente mas larga que las anteriores",
        ];
        let limite = 12;
        assert_eq!(
            cantidad_de_cadenas_mayor_a(&lista, limite),
            3,
            "Debería haber 3 cadenas con más de {limite} caracteres"
        );
        assert_eq!(
            cantidad_de_cadenas_mayor_a(&[], 5),
            0,
            "No hay elementos en la lista, el resultado debería ser 0"
        );
    }
}
