pub fn es_par(num: u64) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeros_pares_son_identificados_correctamente() {
        assert!(es_par(4), "4 debería ser par");
        assert!(es_par(0), "0 debería ser par");
        assert!(!es_par(15), "15 no debería ser par");
        assert!(!es_par(7), "7 no debería ser par");
        assert!(!es_par(u64::MAX), "u64::MAX no debería ser par");
    }
}
