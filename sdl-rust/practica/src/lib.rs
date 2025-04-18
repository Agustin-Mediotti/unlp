mod tp2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeros_pares_son_identificados_correctamente() {
        assert!(tp2::ej1::es_par(4), "4 debería ser par");
        assert!(tp2::ej1::es_par(0), "0 debería ser par");
        assert!(!tp2::ej1::es_par(15), "15 no debería ser par");
        assert!(!tp2::ej1::es_par(7), "7 no debería ser par");
        assert!(!tp2::ej1::es_par(u64::MAX), "u64::MAX no debería ser par");
    }

    #[test]
    fn retorna_true_para_numeros_primos() {
        let primos = [2, 3, 5, 7, 11, 13, 17, 19];
        assert!(
            primos.iter().all(|&n| tp2::ej2::es_primo(n)),
            "Todos los números en la lista deberían ser primos"
        );
        let no_primos = [0, 1, 4, 6, 8, 9, 10, 12, 15];
        assert!(
            no_primos.iter().all(|&n| !tp2::ej2::es_primo(n)),
            "Ninguno de los números en la lista debería ser primo"
        );
    }

    #[test]
    fn calcula_correctamente_la_suma_de_pares() {
        let enteros_pares = [2, 4, 8, 16];
        let enteros_impares = [1, 3, 5, 11];
        assert_eq!(
            tp2::ej3::suma_pares(&enteros_pares),
            30,
            "La suma de los pares [2, 4, 8, 16] debería ser 30"
        );
        assert_eq!(
            tp2::ej3::suma_pares(&enteros_impares),
            0,
            "No hay pares, el resultado debería ser 0"
        );
    }

    #[test]
    fn cuenta_la_cantidad_de_impares_correctamente() {
        let enteros = [1, 2, 3, 5, 8, 11];
        let sin_impares = [0, 2, 4, 8, 12, 18];
        assert_eq!(
            tp2::ej4::cantidad_impares(&enteros),
            4,
            "Debería haber 4 impares en la lista [1, 2, 3, 5, 8, 11]"
        );
        assert_eq!(
            tp2::ej4::cantidad_impares(&sin_impares),
            0,
            "No hay impares en la lista, el resultado debería ser 0"
        );
        assert_eq!(
            tp2::ej4::cantidad_impares(&[]),
            0,
            "No hay elementos en la lista, el resultado debería ser 0"
        );
    }

    #[test]
    fn duplica_los_valores_correctamente() {
        let lista = [1.0, 2.2, 4.8, 5.0, 0.0];
        assert_eq!(
            tp2::ej5::duplicar_valores(&lista),
            [2.0, 4.4, 9.6, 10.0, 0.0],
            "Los valores duplicados no coinciden con lo esperado"
        );
    }

    #[test]
    fn calcula_la_longitud_de_cada_string() {
        let lista = [
            "Hola",
            "UNLP",
            "Informatica",
            "A Don Cangrejo le gusta Rust",
        ];
        assert_eq!(
            tp2::ej6::longitud_de_cadenas(&lista),
            [4, 4, 11, 28],
            "Las longitudes de las cadenas no coinciden con lo esperado"
        );
    }

    #[test]
    fn cuenta_cuantos_elementos_superan_el_limite() {
        let lista = [2, 4, 8, 16, 32, 64];
        let limite = 8;
        assert_eq!(
            tp2::ej7::cantidad_de_mayores(&lista, &limite),
            3,
            "Debería haber 3 elementos mayores que {limite}"
        );
    }

    #[test]
    fn suma_elemento_a_elemento_dos_arreglos() {
        let a = [2.0, 4.2, 8.4, 1.1];
        let b = [2.0, 4.2, 8.4, 1.1];
        assert_eq!(
            tp2::ej8::sumar_arreglos(&a, &b),
            [4.0, 8.4, 16.8, 2.2],
            "La suma de los arreglos no coincide con lo esperado"
        );
    }

    #[test]
    fn cuenta_elementos_en_rango_inclusivo() {
        let lista = [2, 4, 8, 16, 32, 64, 128];
        let inferior = 16;
        let superior = 64;
        assert_eq!(
            tp2::ej9::cantidad_en_rango(&lista, &inferior, &superior),
            3,
            "Debería haber 3 elementos dentro del rango {inferior}..{superior}"
        );
    }

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
            tp2::ej10::cantidad_de_cadenas_mayor_a(&lista, limite),
            3,
            "Debería haber 3 cadenas con más de {limite} caracteres"
        );
        assert_eq!(
            tp2::ej10::cantidad_de_cadenas_mayor_a(&[], 5),
            0,
            "No hay elementos en la lista, el resultado debería ser 0"
        );
    }

    #[test]
    fn multiplica_los_elementos_por_un_factor() {
        let mut lista = [35, 23, 45, 18];
        let factor = 2;
        tp2::ej11::multiplicar_valores(&mut lista, factor);
        assert_eq!(
            lista,
            [70, 46, 90, 36],
            "Los elementos no fueron multiplicados correctamente por el factor"
        );
    }

    #[test]
    fn reemplaza_pares_con_menos_uno() {
        let mut lista = [2, 5, 8, 11, 12, 16];
        tp2::ej12::reemplazar_pares(&mut lista);
        assert_eq!(
            lista,
            [-1, 5, -1, 11, -1, -1],
            "Los números pares no fueron reemplazados correctamente"
        );
    }

    #[test]
    fn ordena_nombres_alfabeticamente() {
        let mut lista = ["ale", "Kim", "92", "Guillermo", "Juan Manuel", "", "Z"];
        tp2::ej13::ordenar_nombres(&mut lista);
        assert_eq!(
            lista,
            ["", "92", "Guillermo", "Juan Manuel", "Kim", "Z", "ale"],
            "Los nombres no fueron ordenados alfabéticamente como se esperaba"
        );
    }

    #[test]
    fn incrementa_variable_de_punto_flotante() {
        let mut num: f32 = 2.0;
        tp2::ej14::incrementar(&mut num);
        assert_eq!(
            num, 3.0,
            "La variable {num} debería haber sido incrementada a 3.0"
        );
    }
}
