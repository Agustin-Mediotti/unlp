use super::ej3::Fecha;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum EstadoPrestamo {
    Devuelto,
    Prestamo,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum GeneroLibro {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

pub struct Biblioteca {
    pub mombre: String,
    pub direccion: String,
    pub prestamos_efectuados: Vec<Prestamo>,
    pub libros_a_disposicion: HashMap<Libro, u8>,
    pub fecha: Fecha,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Libro {
    pub isbn: u32,
    pub titulo: String,
    pub autor: String,
    pub numero_de_paginas: u32,
    pub genero: GeneroLibro,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: u32,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Prestamo {
    pub libro: Libro,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>,
    pub cliente: Cliente,
    pub estado: EstadoPrestamo,
}

impl Biblioteca {
    /// Dado un determinado libro, devuelve la cantidad de copias a disposición para dicho libro.
    pub fn obtener_cantidad_de_copias(&self, libro: Libro) -> u8 {
        if let Some(copias) = self.libros_a_disposicion.get(&libro) {
            *copias
        } else {
            0
        }
    }

    /// Dado un determinado libro, decrementa la cantidad de copias a disposición.
    pub fn decrementar_copias_a_disposicion(&mut self, libro: Libro) {
        if let Some(copias) = self.libros_a_disposicion.get_mut(&libro) {
            if *copias > 0 {
                *copias -= 1;
            }
        }
    }

    /// Dado un determinado libro, incrementa la cantidad de copias a disposición.
    pub fn incrementar_copias_a_disposicion(&mut self, libro: Libro) {
        if let Some(copias) = self.libros_a_disposicion.get_mut(&libro) {
            if *copias < u8::max_value() {
                *copias += 1;
            }
        } else {
            self.libros_a_disposicion
                .entry(libro.clone())
                .insert_entry(1);
        }
    }

    /// Devuelve la cantidad de prestamos en estado "EstadoPrestamo::Prestamo" de un determinado Cliente.
    pub fn contar_prestamos_de_cliente(&self, cliente: Cliente) -> usize {
        self.prestamos_efectuados
            .iter()
            .filter(|f| f.cliente == cliente && f.estado == EstadoPrestamo::Prestamo)
            .count()
    }

    /// Dado un determinado cliente, realiza un prestamo de un libro cumpliendo lo siguiente:
    /// El cliente no debe tener más de 5 prestamos en estado "EstadoPrestamo::Prestamo".
    /// El registro de copias debe contener una copia a disposición.
    pub fn realizar_prestamo_a_cliente(&mut self, cliente: Cliente, libro: Libro) -> bool {
        if self
            .prestamos_efectuados
            .iter()
            .filter(|f| f.cliente == cliente && f.estado == EstadoPrestamo::Prestamo)
            .count()
            < 5
        {
            if self.obtener_cantidad_de_copias(libro.clone()) > 0 {
                self.decrementar_copias_a_disposicion(libro.clone());
                let mut fecha_vencimiento = self.fecha;
                fecha_vencimiento.sumar_dias(15);
                self.prestamos_efectuados.push(Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento,
                    fecha_devolucion: None,
                    cliente,
                    estado: EstadoPrestamo::Prestamo,
                });
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Dado una cantidad de días, devuelve un vector de Prestamos a vencer en el plazo.
    pub fn ver_prestamos_a_vencer(&self, dias: u8, fecha_actual: Fecha) -> Vec<Prestamo> {
        self.prestamos_efectuados
            .iter()
            .filter(|f| f.estado == EstadoPrestamo::Prestamo)
            .filter(|f| {
                let mut fecha_limite = fecha_actual;
                fecha_limite.sumar_dias(dias as i64);
                f.fecha_vencimiento <= fecha_limite
            })
            .cloned()
            .collect()
    }

    /// Dada una fecha, devuelve un vector de Prestamos vencidos.
    pub fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<Prestamo> {
        self.prestamos_efectuados
            .iter()
            .filter(|f| f.estado == EstadoPrestamo::Prestamo)
            .filter(|f| f.fecha_vencimiento <= fecha_actual)
            .cloned()
            .collect()
    }

    /// Dado un Libro y un Cliente, se busca un préstamo y se retorna si existe.
    pub fn buscar_prestamo(&self, cliente: Cliente, libro: Libro) -> Option<Prestamo> {
        if let Some(prestamo) = self
            .prestamos_efectuados
            .iter()
            .find(|f| f.libro == libro && f.cliente == cliente)
            .cloned()
        {
            Some(prestamo)
        } else {
            None
        }
    }
    /// Dado un Libro y un Cliente, se busca el prestamo y cambia de estado a "EstadoPrestamo::Devuelto".
    /// Se incrementa la fecha de devolución y se incrementa la cantidad de copias a disposición.
    pub fn devolver_libro(&mut self, cliente: Cliente, libro: Libro) {
        if let Some(prestamo) = self
            .prestamos_efectuados
            .iter_mut()
            .find(|f| f.cliente == cliente && f.libro == libro)
        {
            prestamo.estado = EstadoPrestamo::Devuelto;
            self.incrementar_copias_a_disposicion(libro);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_fecha_base() -> Fecha {
        Fecha::new(02, 05, 2025).unwrap()
    }

    fn build_libro_base() -> Libro {
        Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        }
    }

    fn build_libro_02() -> Libro {
        Libro {
            isbn: 123,
            titulo: "El libro que no existe".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 320,
            genero: GeneroLibro::Novela,
        }
    }

    fn build_biblio() -> Biblioteca {
        Biblioteca {
            mombre: "Biblio".to_string(),
            direccion: "Calle 1".to_string(),
            prestamos_efectuados: vec![],
            libros_a_disposicion: HashMap::new(),
            fecha: build_fecha_base(),
        }
    }

    fn build_biblio_con_prestamo() -> Biblioteca {
        Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: build_cliente(),
                estado: EstadoPrestamo::Prestamo,
            }],
            libros_a_disposicion: HashMap::new(),
            fecha: build_fecha_base(),
        }
    }

    fn build_biblio_con_prestamos() -> Biblioteca {
        Biblioteca {
            fecha: build_fecha_base(),
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Prestamo,
                },
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Devuelto,
                },
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Prestamo,
                },
            ],
            libros_a_disposicion: HashMap::new(),
        }
    }

    fn build_biblio_con_prestamos_devueltos() -> Biblioteca {
        Biblioteca {
            fecha: build_fecha_base(),
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Prestamo,
                },
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Devuelto,
                },
                Prestamo {
                    libro: build_libro_base(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                    cliente: build_cliente(),
                    estado: EstadoPrestamo::Prestamo,
                },
            ],
            libros_a_disposicion: HashMap::from([(build_libro_base(), 0)]),
        }
    }

    fn build_cliente() -> Cliente {
        Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        }
    }

    #[test]
    fn obtener_cantidad_de_copias_correctamente() {
        let mut biblio = build_biblio();
        let libro = build_libro_base();

        assert_eq!(biblio.obtener_cantidad_de_copias(libro.clone()), 0);
        biblio.libros_a_disposicion.insert(libro.clone(), 3);
        assert_eq!(biblio.obtener_cantidad_de_copias(libro), 3);
    }

    #[test]
    fn incrementar_y_decrementar_copias_correctamente() {
        let mut biblio = build_biblio();
        let libro = build_libro_base();
        biblio.libros_a_disposicion.insert(libro.clone(), 1);
        biblio.incrementar_copias_a_disposicion(libro.clone());

        assert_eq!(biblio.obtener_cantidad_de_copias(libro.clone()), 2);
        biblio.decrementar_copias_a_disposicion(libro.clone());
        assert_eq!(biblio.obtener_cantidad_de_copias(libro), 1);
    }

    #[test]
    fn ver_prestamos_a_vencer_correctamente() {
        let biblio = build_biblio_con_prestamo();
        assert_eq!(
            biblio.ver_prestamos_a_vencer(15, Fecha::new(01, 05, 2025).unwrap()),
            [Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: build_cliente(),
                estado: EstadoPrestamo::Prestamo,
            }]
        )
    }

    #[test]
    fn contar_prestamos_cliente_correctamente() {
        let biblio = build_biblio_con_prestamo();
        let cliente = build_cliente();
        assert_eq!(biblio.contar_prestamos_de_cliente(cliente), 1);
    }

    #[test]
    fn realizar_prestamo_a_cliente_correctamente() {
        let mut biblio = build_biblio();
        let cliente = build_cliente();
        let libro = build_libro_base();

        assert!(
            !biblio.realizar_prestamo_a_cliente(cliente.clone(), libro.clone()),
            "No se debe poder realizar un prestamo a un cliente si la cantidad de copias es menor a 1"
        );
        biblio.incrementar_copias_a_disposicion(libro.clone());

        biblio.realizar_prestamo_a_cliente(cliente.clone(), libro.clone());
        assert_eq!(
            biblio.prestamos_efectuados.len(),
            1,
            "La cantidad de prestamos efectuados esperada no es correcta"
        );

        for _ in 1..=4 {
            biblio.incrementar_copias_a_disposicion(libro.clone());
            assert!(
                biblio.realizar_prestamo_a_cliente(cliente.clone(), libro.clone()),
                "No se realizo un prestamo a un cliente como se esperaba"
            );
        }

        biblio.incrementar_copias_a_disposicion(libro.clone());
        assert!(
            !biblio.realizar_prestamo_a_cliente(cliente.clone(), libro.clone()),
            "No se debe poder realizar un prestamo a un cliente con mas de 5 prestamos activos"
        );
    }

    #[test]
    fn ver_prestamos_vencidos_correctamente() {
        let biblio = build_biblio_con_prestamos();
        let vencidos = biblio.ver_prestamos_vencidos(Fecha::new(02, 05, 2025).unwrap());
        assert_eq!(vencidos.len(), 2);
    }

    #[test]
    fn buscar_prestamo_correctamente() {
        let biblio = build_biblio_con_prestamos();
        let cliente = build_cliente();
        let libro = build_libro_base();
        let libro_inexistente = build_libro_02();
        assert!(biblio.buscar_prestamo(cliente.clone(), libro).is_some());
        assert!(biblio.buscar_prestamo(cliente, libro_inexistente).is_none());
    }

    #[test]
    fn devolver_libro_correctamente() {
        let cliente = build_cliente();
        let libro = build_libro_base();
        let mut biblio = build_biblio_con_prestamos_devueltos();

        biblio.devolver_libro(cliente, libro.clone());
        assert_eq!(
            biblio.obtener_cantidad_de_copias(libro),
            1,
            "Se esperan cantidad de copias 1"
        );
        assert_eq!(
            biblio.prestamos_efectuados[0].estado,
            EstadoPrestamo::Devuelto
        );
    }
}
