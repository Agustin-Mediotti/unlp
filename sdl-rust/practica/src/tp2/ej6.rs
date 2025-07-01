pub fn longitud_de_cadenas(lista: &[&str]) -> Vec<u32> {
    lista.iter().map(|e| e.chars().count() as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_la_longitud_de_cada_string() {
        let lista = [
            "Hola",
            "UNLP",
            "Informatica",
            "A Don Cangrejo le gusta Rust",
        ];
        assert_eq!(
            longitud_de_cadenas(&lista),
            [4, 4, 11, 28],
            "Las longitudes de las cadenas no coinciden con lo esperado"
        );
    }
}
