pub fn cantidad_en_rango(lista: &[i32], inferior: &i32, superior: &i32) -> u32 {
    lista
        .iter()
        .filter(|x| x >= &inferior && x <= &superior)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuenta_elementos_en_rango_inclusivo() {
        let lista = [2, 4, 8, 16, 32, 64, 128];
        let inferior = 16;
        let superior = 64;
        assert_eq!(
            cantidad_en_rango(&lista, &inferior, &superior),
            3,
            "Debería haber 3 elementos dentro del rango {inferior}..{superior}"
        );
    }
}
