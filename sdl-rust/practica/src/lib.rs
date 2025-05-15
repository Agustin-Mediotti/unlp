mod tp2;
mod tp3;

#[cfg(test)]
mod tests {
    use crate::tp3::{
        ej2::Rectangulo,
        ej3::Fecha,
        ej4::{Triangulo, TrianguloTipo},
        ej5::Producto,
        ej6::{Estudiante, Examen},
        ej7::{Auto, ColorAuto, ConcesionarioAuto},
        ej8::{Cancion, Genero, Playlist},
    };

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

    #[test]
    fn crea_nueva_persona_sin_direccion() {
        let persona = tp3::ej1::Persona::new("Fausto".to_string(), 11, None);
        assert_eq!(persona.nombre, "Fausto");
        assert_eq!(persona.edad, 11);
        assert_eq!(persona.dir, None);
    }

    #[test]
    fn crea_nueva_persona_con_direccion() {
        let persona = tp3::ej1::Persona::new(
            "Fausto".to_string(),
            11,
            Some("Calle Falsa 123".to_string()),
        );
        assert_eq!(persona.nombre, "Fausto");
        assert_eq!(persona.edad, 11);
        assert_eq!(persona.dir, Some("Calle Falsa 123".to_string()));
    }

    #[test]
    fn persona_to_string_con_direccion() {
        let persona_to_string = tp3::ej1::Persona::new(
            "Fausto".to_string(),
            11,
            Some("Calle Falsa 123".to_string()),
        )
        .to_string();

        assert_eq!(persona_to_string, "Fausto, 11, Calle Falsa 123")
    }

    #[test]
    fn obtener_edad_de_persona() {
        let persona = tp3::ej1::Persona::new(
            "Fausto".to_string(),
            11,
            Some("Calle Falsa 123".to_string()),
        );

        assert_eq!(persona.obtener_edad(), 11);
    }

    #[test]
    fn actualizar_direccion_de_persona() {
        let mut persona = tp3::ej1::Persona::new("Fausto".to_string(), 11, None);
        persona.actualizar_direccion("Fausto, 11, Calle Falsa 123".to_string());

        assert_eq!(persona.dir, Some("Fausto, 11, Calle Falsa 123".to_string()));
    }

    #[test]
    fn crea_rectangulo_con_parametros_correctos() {
        let rect = Rectangulo::new(5, 5);

        assert_eq!(rect.ancho, 5);
        assert_eq!(rect.longitud, 5);
    }

    #[test]
    fn calcular_correctamente_area_de_rectangulo() {
        assert_eq!(Rectangulo::new(2, 0).calcular_area(), 0);
        assert_eq!(Rectangulo::new(1, 1).calcular_area(), 1);
        assert_eq!(Rectangulo::new(5, 6).calcular_area(), 30);
    }

    #[test]
    fn calcular_correctamente_perimetro_de_rectangulo() {
        assert_eq!(Rectangulo::new(2, 0).calcular_perimetro(), 0);
        assert_eq!(Rectangulo::new(1, 1).calcular_perimetro(), 2);
        assert_eq!(Rectangulo::new(5, 6).calcular_perimetro(), 60);
    }

    #[test]
    fn identifica_cuadrado_correctamente() {
        assert!(!Rectangulo::new(2, 0).es_cuadrado());
        assert!(Rectangulo::new(1, 1).es_cuadrado());
        assert!(!Rectangulo::new(5, 6).es_cuadrado());
        assert!(Rectangulo::new(u32::max_value(), u32::max_value()).es_cuadrado());
    }

    #[test]
    fn crea_una_fecha_con_correctamente() {
        assert!(Fecha::new(11, 09, 2001).is_some());
        assert!(Fecha::new(31, 11, 2025).is_none());
        assert!(Fecha::new(29, 2, 2023).is_none());
    }

    #[test]
    fn verifica_fecha_valida() {
        assert!(!Fecha::es_fecha_valida(29, 2, 2023));
        assert!(!Fecha::es_fecha_valida(31, 11, 2025));
        assert!(Fecha::es_fecha_valida(31, 12, 2025));
    }

    #[test]
    fn verifica_fecha_bisiesto() {
        assert!(
            Fecha::new(29, 3, 2024)
                .expect("Fecha incorrecta")
                .es_bisiesto()
        );
        assert!(
            !Fecha::new(19, 5, 2023)
                .expect("Fecha incorrecta")
                .es_bisiesto()
        );
    }

    #[test]
    fn verifica_sumar_dias_a_fecha() {
        let mut fecha = Fecha::new(11, 09, 2001).unwrap();
        fecha.sumar_dias(15);

        assert_eq!(fecha, Fecha::new(26, 09, 2001).unwrap());
    }

    #[test]
    fn verifica_restar_dias_a_fecha() {
        let mut fecha = Fecha::new(11, 09, 2001).unwrap();
        fecha.restar_dias(2);

        assert_eq!(fecha, Fecha::new(09, 09, 2001).unwrap());
    }

    #[test]
    fn crea_nuevo_triangulo_correctamente() {
        assert_eq!(Triangulo::new(5, 5, 5), Triangulo { x: 5, y: 5, z: 5 });
        assert_ne!(Triangulo::new(2, 7, 3), Triangulo { x: 1, y: 1, z: 1 });
    }

    #[test]
    fn determinar_tipo_triangulo_correctamente() {
        assert_eq!(
            Triangulo::new(5, 5, 5).determinar_tipo().unwrap(),
            TrianguloTipo::Equilatero
        );
        assert_eq!(
            Triangulo::new(2, 7, 1).determinar_tipo().unwrap(),
            TrianguloTipo::Escaleno
        );

        assert_eq!(
            Triangulo::new(2, 2, 1).determinar_tipo().unwrap(),
            TrianguloTipo::Isosceles
        );
    }

    #[test]
    fn calcula_correctamente_el_area_del_rectangulo() {
        assert_eq!(
            Triangulo::new(5, 5, 5).calcular_area(),
            (f64::sqrt(3.0) / 4.0) * (5.0_f64.powf(2.0))
        );
        assert_eq!(
            Triangulo::new(7, 2, 6).calcular_area(),
            (f64::sqrt(3.0) / 4.0) * (2.0_f64.powf(2.0))
        );
    }

    #[test]
    fn calcular_correctamente_perimetro_del_triangulo() {
        assert_eq!(Triangulo::new(5, 5, 5).calcular_perimetro(), 15);
        assert_eq!(Triangulo::new(2, 7, 9).calcular_perimetro(), 18);
        assert_eq!(Triangulo::new(2, 3, 5).calcular_perimetro(), 10);
    }

    #[test]
    fn crea_nuevo_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192),
            Producto {
                nombre: "Nodemcu Esp32".to_string(),
                precio_bruto: 14990.00,
                numero_ident: 3032192
            }
        );
    }

    #[test]
    fn calcula_impuestos_de_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192).calcular_impuestos(20.00),
            14990.00 * (20.00 / 100.0)
        );
    }

    #[test]
    fn calcula_descuento_de_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192).aplicar_descuento(35.00),
            14990.00 * (35.00 / 100.0)
        );
    }

    #[test]
    fn calcula_correctamente_precio_total_de_producto() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(Some(20.00), None),
            14990.00 + (14990.00 * (20.00 / 100.0)),
            "El precio total no se calculo como se esperaba (20% impuestos y 0% descuento)"
        );

        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(None, Some(35.00)),
            14990.00 - (14990.00 * (35.00 / 100.0)),
            "El precio total no se calculo como se esperaba (0% impuestos y 35% descuento)"
        );

        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(Some(20.00), Some(35.00)),
            14990.00 - (14990.00 * (35.00 / 100.0)) + (14990.00 * (20.00 / 100.0)),
            "El precio total no se calculo como se esperaba (20% impuestos y 35% descuento)"
        );
    }

    #[test]
    fn crea_nuevo_examen_correctamente() {
        assert_eq!(
            Examen::new("Matemática".to_owned(), 8),
            Examen {
                nombre_materia: "Matemática".to_owned(),
                nota: 8
            },
            "El objeto Examen no se creo como se esperaba"
        );
    }

    #[test]
    fn crea_nuevo_estudiante_correctamente() {
        assert_eq!(
            Estudiante::new("Calamardo".to_owned(), 42, Vec::new()),
            Estudiante {
                nombre: "Calamardo".to_owned(),
                numero_ident: 42,
                calificaciones: Vec::new(),
            },
            "El objeto Estudiante no se creo como se esperaba"
        );
    }

    #[test]
    fn obtiene_promedio_correctamente() {
        let mut calificaciones: Vec<Examen> = Vec::new();
        calificaciones.push(Examen::new("Matemática".to_owned(), 8));
        calificaciones.push(Examen::new("Historia".to_owned(), 10));
        let estudiante = Estudiante::new("Calamardo".to_owned(), 42, calificaciones);

        assert_eq!(
            estudiante.obtener_promedio(),
            9.0,
            "El promedio no se calculo como se esperaba"
        );
    }

    #[test]
    fn obtiene_calificacion_mas_alta_correctamente() {
        let mut calificaciones: Vec<Examen> = Vec::new();
        calificaciones.push(Examen::new("Matemática".to_owned(), 8));
        calificaciones.push(Examen::new("Historia".to_owned(), 10));
        let estudiante = Estudiante::new("Calamardo".to_owned(), 42, calificaciones);

        assert_eq!(
            estudiante.obtener_calificacion_mas_alta(),
            10,
            "No se obtubo el elemento esperado"
        );
    }

    #[test]
    fn obtiene_calificacion_mas_baja_correctamente() {
        let mut calificaciones: Vec<Examen> = Vec::new();
        calificaciones.push(Examen::new("Matemática".to_owned(), 8));
        calificaciones.push(Examen::new("Historia".to_owned(), 10));
        let estudiante = Estudiante::new("Calamardo".to_owned(), 42, calificaciones);

        assert_eq!(
            estudiante.obtener_calificacion_mas_baja(),
            8,
            "No se obtubo el elemento esperado"
        );
    }

    #[test]
    fn crea_nuevo_concesionario_auto_correctamente() {
        assert_eq!(
            ConcesionarioAuto::new(
                "Concesionario 44".to_owned(),
                "44 13 y 14".to_owned(),
                5,
                Vec::new()
            ),
            ConcesionarioAuto {
                nombre: "Concesionario 44".to_owned(),
                direccion: "44 13 y 14".to_owned(),
                capacidad_max: 5,
                autos: Vec::new()
            },
            "No se creo el objeto como se esperaba"
        );
    }

    #[test]
    fn agregar_auto_a_concesionario_correctamente() {
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        });
        assert_eq!(
            conce.autos.len(),
            1,
            "No se agrego el auto al concesionario como se esperaba"
        );
    }

    #[test]
    fn eliminar_auto_de_concesionario_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(auto.clone());
        assert_eq!(conce.autos.len(), 1,);
        conce.eliminar_auto(auto);
        assert_eq!(
            conce.autos.len(),
            0,
            "No se elimino el auto al concesionario como se esperaba"
        );
    }

    #[test]
    fn buscar_auto_en_concesionario_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(auto.clone());

        assert_eq!(
            conce.buscar_auto(auto.clone()),
            Some(auto),
            "No se encontro el auto en el concesionario como se esperaba"
        );
    }

    #[test]
    fn calcular_precio_de_auto_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };

        assert_eq!(
            auto.calcular_precio(),
            40_500.0,
            "No se calculo el precio del auto como se esperaba"
        );
    }

    #[test]
    fn agrega_cancion_a_playlist_correctamente() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());

        assert_eq!(
            playlist.canciones[0], cancion,
            "No se agrego la cancion a la playlist como se esperaba"
        );
    }

    #[test]
    fn elimina_cancion_de_playlist_correctamente() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());
        playlist.eliminar_cancion(cancion);

        assert_eq!(
            playlist.canciones.len(),
            0,
            "No se elimino la cancion de la playlist como se esperaba"
        );
    }

    #[test]
    fn mueve_cancion_a_una_determinada_posicion_de_la_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2);

        playlist.mover_cancion(cancion_1.clone(), 1);

        assert_eq!(
            playlist.canciones[1], cancion_1,
            "No se movio la cancion de la playlist a la posicion que se esperaba"
        );
    }

    #[test]
    fn busca_cancion_por_nombre_en_playlist() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());

        assert_eq!(
            playlist
                .buscar_cancion_por_nombre(cancion.clone())
                .unwrap()
                .to_owned(),
            cancion,
            "No se encontro la cancion en la playlist como se esperaba"
        );
    }

    #[test]
    fn obtener_canciones_por_genero_en_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        let mut expected = Vec::<&Cancion>::new();
        expected.push(&cancion_1);
        expected.push(&cancion_2);

        assert_eq!(
            playlist.obtener_canciones_por_genero(Genero::Otros),
            expected,
            "No se obtuvieron las canciones de la playlist por genero como se esperaba"
        );
    }

    #[test]
    fn obtener_canciones_por_artista_en_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        let mut expected = Vec::<&Cancion>::new();
        expected.push(&cancion_2);

        assert_eq!(
            playlist.obtener_canciones_por_artista("Lacuna Coil".to_owned()),
            expected,
            "No se obtuvieron las canciones de la playlist por artista como se esperaba"
        );
    }

    #[test]
    fn modificar_titulo_de_playlist() {
        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.modificar_titulo("Metal only".to_owned());

        assert_eq!(
            playlist.nombre,
            "Metal only".to_owned(),
            "No se cambio el título de la playlist como se esperaba"
        );
    }

    #[test]
    fn eliminar_canciones_de_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        playlist.limpiar_playlist();

        assert_eq!(
            playlist.canciones,
            Vec::<Cancion>::new(),
            "No se eliminaron todas las canciones de la playlist como se esperaba"
        );
    }
}
