use super::ej3::Fecha;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum EstadoPrestamo {
    Devuelto,
    Prestamo,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum GeneroLibro {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct Prestamo {
    pub libro: Libro,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Fecha,
    pub cliente: Cliente,
    pub estado: EstadoPrestamo,
}

#[allow(dead_code)]
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
