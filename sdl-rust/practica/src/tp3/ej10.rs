use super::ej3::Fecha;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum EstadoPrestamo {
    Devuelto,
    Prestamo,
}

#[derive(PartialEq, Eq, Hash, Clone)]
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
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Libro {
    pub isbn: u32,
    pub titulo: String,
    pub autor: String,
    pub numero_de_paginas: u32,
    pub genero: GeneroLibro,
}

#[derive(PartialEq, Clone)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: u32,
    pub email: String,
}

#[derive(Clone)]
pub struct Prestamo {
    pub libro: Libro,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Fecha,
    pub cliente: Cliente,
    pub estado: EstadoPrestamo,
}

impl Biblioteca {
    pub fn obtener_cantidad_de_copias(&self, libro: Libro) -> u8 {
        if let Some(copias) = self.libros_a_disposicion.get(&libro) {
            *copias
        } else {
            0
        }
    }

    pub fn decrementar_copias_a_disposicion(&mut self, libro: Libro) {
        if let Some(copias) = self.libros_a_disposicion.get_mut(&libro) {
            if *copias > 0 {
                *copias -= 1;
            }
        }
    }

    pub fn incrementar_copias_a_disposicion(&mut self, libro: Libro) {
        if let Some(copias) = self.libros_a_disposicion.get_mut(&libro) {
            if *copias < u8::max_value() {
                *copias += 1;
            }
        }
    }

    pub fn contar_prestamos_de_cliente(&self, cliente: Cliente) -> usize {
        self.prestamos_efectuados
            .iter()
            .filter(|f| f.cliente == cliente && f.estado == EstadoPrestamo::Prestamo)
            .count()
    }

    pub fn realizar_prestamo_a_cliente(&mut self, cliente: Cliente, libro: Libro) -> bool {
        if self
            .prestamos_efectuados
            .iter()
            .filter(|f| f.cliente == cliente && f.estado == EstadoPrestamo::Prestamo)
            .count()
            < 5
        {
            self.incrementar_copias_a_disposicion(libro);
            true
        } else {
            false
        }
    }

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

    pub fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<Prestamo> {
        self.prestamos_efectuados
            .iter()
            .filter(|f| f.estado == EstadoPrestamo::Prestamo)
            .filter(|f| f.fecha_vencimiento <= fecha_actual)
            .cloned()
            .collect()
    }

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

    #[test]
    fn obtener_cantidad_de_copias_correctamente() {
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let mut map = HashMap::new();
        map.insert(libro.clone(), 3);
        let biblio = Biblioteca {
            mombre: "Biblio".to_string(),
            direccion: "Calle 1".to_string(),
            prestamos_efectuados: vec![],
            libros_a_disposicion: map,
        };
        assert_eq!(biblio.obtener_cantidad_de_copias(libro), 3);
    }

    #[test]
    fn incrementar_y_decrementar_copias_correctamente() {
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let mut map = HashMap::new();
        map.insert(libro.clone(), 1);
        let mut biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![],
            libros_a_disposicion: map,
        };
        biblio.incrementar_copias_a_disposicion(libro.clone());
        assert_eq!(biblio.obtener_cantidad_de_copias(libro.clone()), 2);
        biblio.decrementar_copias_a_disposicion(libro.clone());
        assert_eq!(biblio.obtener_cantidad_de_copias(libro.clone()), 1);
    }

    #[test]
    fn contar_prestamos_cliente_correctamente() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        };
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![Prestamo {
                libro,
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            }],
            libros_a_disposicion: HashMap::new(),
        };
        assert_eq!(biblio.contar_prestamos_de_cliente(cliente), 1);
    }

    #[test]
    fn realizar_prestamo_a_cliente_correctamente() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        };
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let mut biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![Prestamo {
                libro: libro.clone(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            }],
            libros_a_disposicion: HashMap::new(),
        };
        biblio.realizar_prestamo_a_cliente(cliente, libro);
        assert_eq!(biblio.prestamos_efectuados.len(), 1);
    }

    #[test]
    fn ver_prestamos_vencidos_correctamente() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        };
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Devuelto,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
            ],
            libros_a_disposicion: HashMap::new(),
        };
        let vencidos = biblio.ver_prestamos_vencidos(Fecha::new(02, 05, 2025).unwrap());
        assert_eq!(vencidos.len(), 2);
    }

    #[test]
    fn buscar_prestamo_correctamente() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        };
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Devuelto,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
            ],
            libros_a_disposicion: HashMap::new(),
        };
        let found = biblio.buscar_prestamo(cliente, libro);
        assert!(found.is_some());
    }

    #[test]
    fn devolver_libro_correctamente() {
        let cliente = Cliente {
            nombre: "Juan".to_string(),
            telefono: 123456,
            email: "juan@email.com".to_string(),
        };
        let libro = Libro {
            isbn: 123,
            titulo: "El libro".to_string(),
            autor: "Autor".to_string(),
            numero_de_paginas: 100,
            genero: GeneroLibro::Novela,
        };
        let mut biblio = Biblioteca {
            mombre: "B".to_string(),
            direccion: "X".to_string(),
            prestamos_efectuados: vec![
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Devuelto,
                },
                Prestamo {
                    libro: libro.clone(),
                    fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                    fecha_devolucion: Fecha::new(01, 05, 2025).unwrap(),
                    cliente: cliente.clone(),
                    estado: EstadoPrestamo::Prestamo,
                },
            ],
            libros_a_disposicion: HashMap::from([(libro.clone(), 0)]),
        };
        biblio.devolver_libro(cliente.clone(), libro.clone());
        assert_eq!(biblio.obtener_cantidad_de_copias(libro.clone()), 1);
        assert_eq!(
            biblio.prestamos_efectuados[0].estado,
            EstadoPrestamo::Devuelto
        );
    }
}
