pub fn reemplazar_pares(lista: &mut [i32]) {
    lista
        .iter_mut()
        .filter(|e| **e % 2 == 0)
        .for_each(|e| *e = -1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reemplaza_pares_con_menos_uno() {
        let mut lista = [2, 5, 8, 11, 12, 16];
        reemplazar_pares(&mut lista);
        assert_eq!(
            lista,
            [-1, 5, -1, 11, -1, -1],
            "Los números pares no fueron reemplazados correctamente"
        );
    }
}
