pub fn es_primo(num: u64) -> bool {
    match num {
        0 | 1 => false,
        2 | 3 => true,
        _ => !(2..=((num as f64).sqrt() as u64)).any(|i| num % i == 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_true_para_numeros_primos() {
        let primos = [2, 3, 5, 7, 11, 13, 17, 19];
        assert!(
            primos.iter().all(|&n| es_primo(n)),
            "Todos los números en la lista deberían ser primos"
        );
        let no_primos = [0, 1, 4, 6, 8, 9, 10, 12, 15];
        assert!(
            no_primos.iter().all(|&n| !es_primo(n)),
            "Ninguno de los números en la lista debería ser primo"
        );
    }
}
