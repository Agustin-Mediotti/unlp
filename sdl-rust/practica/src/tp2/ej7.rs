pub fn cantidad_de_mayores(lista: &[u32], limite: &u32) -> u32 {
    lista.iter().filter(|e| e > &limite).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuenta_cuantos_elementos_superan_el_limite() {
        let lista = [2, 4, 8, 16, 32, 64];
        let limite = 8;
        assert_eq!(
            cantidad_de_mayores(&lista, &limite),
            3,
            "Debería haber 3 elementos mayores que {limite}"
        );
    }
}
