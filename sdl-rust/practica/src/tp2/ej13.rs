pub fn ordenar_nombres(list: &mut [&str]) {
    list.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_nombres_alfabeticamente() {
        let mut lista = ["ale", "Kim", "92", "Guillermo", "Juan Manuel", "", "Z"];
        ordenar_nombres(&mut lista);
        assert_eq!(
            lista,
            ["", "92", "Guillermo", "Juan Manuel", "Kim", "Z", "ale"],
            "Los nombres no fueron ordenados alfabéticamente como se esperaba"
        );
    }
}
