pub fn multiplicar_valores(lista: &mut [i32], factor: i32) {
    lista.iter_mut().for_each(|x| *x *= factor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplica_los_elementos_por_un_factor() {
        let mut lista = [35, 23, 45, 18];
        let factor = 2;
        multiplicar_valores(&mut lista, factor);
        assert_eq!(
            lista,
            [70, 46, 90, 36],
            "Los elementos no fueron multiplicados correctamente por el factor"
        );
    }
}
