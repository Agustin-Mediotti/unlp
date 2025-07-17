use crate::ent::entregable_2::fecha::Fecha;
use rand::Rng;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

mod fecha {
    use serde::Serialize;
    use time::{
        Duration, {Date, Month},
    };

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
    pub struct Fecha {
        fecha: Date,
    }

    impl Fecha {
        pub fn new(dia: u8, mes: u8, anio: i32) -> Option<Self> {
            if Self::es_fecha_valida(dia, mes, anio) {
                Some(Self {
                    fecha: Date::from_calendar_date(anio, Month::try_from(mes).unwrap(), dia)
                        .unwrap(),
                })
            } else {
                None
            }
        }

        pub fn es_fecha_valida(dia: u8, mes: u8, anio: i32) -> bool {
            if let Ok(month) = Month::try_from(mes) {
                Date::from_calendar_date(anio, month, dia).is_ok()
            } else {
                false
            }
        }

        pub fn es_bisiesto(&self) -> bool {
            /*
                https://es.wikipedia.org/wiki/A%C3%B1o_bisiesto

               p: Es divisible por 4
               q: Es divisible por 100
               r: Es divisible por 400

               (p ∧ ~q) ∨ r
            */
            (self.fecha.year() % 4 == 0 && self.fecha.year() & 100 != 0)
                || self.fecha.year() & 400 == 0
        }

        pub fn sumar_dias(&mut self, dias: i64) {
            self.fecha = self.fecha + Duration::days(dias);
        }

        pub fn restar_dias(&mut self, dias: i64) {
            self.fecha = self.fecha - Duration::days(dias);
        }

        pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
            self.fecha > una_fecha.fecha
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
    }
}

const FILE_NAME: &str = "src/tp5/archivo_biblioteca.json";

#[derive(PartialEq, Clone, Debug, Serialize)]
pub enum EstadoPrestamo {
    Devuelto,
    Prestamo,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FiltroEstadoPrestamo {
    Devuelto,
    Prestamo,
    Todos,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize)]
pub enum GeneroLibro {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Serialize)]
pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub prestamos_efectuados: Vec<Prestamo>,
    #[serde(serialize_with = "as_pairs")]
    pub libros_a_disposicion: HashMap<Libro, u8>,
    pub fecha: Fecha,
    pub clientes: HashMap<String, Cliente>,
}

fn as_pairs<S>(map: &HashMap<Libro, u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<(&Libro, &u8)> = map.iter().collect();
    vec.serialize(serializer)
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize)]
pub struct Libro {
    pub isbn: u32,
    pub titulo: String,
    pub autor: String,
    pub numero_de_paginas: u32,
    pub genero: GeneroLibro,
}

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Cliente {
    pub id_cliente: String,
    pub nombre: String,
    pub telefono: u32,
    pub email: String,
}

impl Cliente {
    /// Genera una nueva instancia de Cliente.
    fn new(nombre: String, telefono: u32, email: String) -> Self {
        Cliente {
            id_cliente: Cliente::generar_id(),
            nombre,
            telefono,
            email,
        }
    }

    /// Genera un String de un ID aleatorio.
    fn generar_id() -> String {
        format!("{}", rand::rng().random::<u32>())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Prestamo {
    pub libro: Libro,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>,
    pub cliente: Cliente,
    pub estado: EstadoPrestamo,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResultadoFiltro {
    pub isbn: u32,
    pub titulo: String,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>,
    pub estado: EstadoPrestamo,
}

impl Biblioteca {
    /// # Recuperatorio Entregable 2: v2
    /// Dado un ID de cliente y un filtro de estado, devuelve la coleccion de préstamos de cliente segun estado indicado.
    pub fn get_historial_prestamos(
        &self,
        id_cliente: String,
        filtro_estado: FiltroEstadoPrestamo,
    ) -> Vec<ResultadoFiltro> {
        self.prestamos_efectuados
            .iter()
            .filter(|e| {
                e.cliente.id_cliente == id_cliente
                    && match filtro_estado {
                        FiltroEstadoPrestamo::Todos => true,
                        FiltroEstadoPrestamo::Prestamo => e.estado == EstadoPrestamo::Prestamo,
                        FiltroEstadoPrestamo::Devuelto => e.estado == EstadoPrestamo::Devuelto,
                    }
            })
            .map(|e| ResultadoFiltro {
                isbn: e.libro.isbn,
                titulo: e.libro.titulo.clone(),
                fecha_vencimiento: e.fecha_vencimiento,
                fecha_devolucion: e.fecha_devolucion,
                estado: e.estado.clone(),
            })
            .collect()
    }

    /// Genera una instacia de Biblioteca y genera el archivo de persistencia de datos.
    pub fn new(direccion: String, mombre: String, fecha: Fecha) -> Self {
        let mut file = File::create(FILE_NAME).unwrap();
        let biblio = Biblioteca {
            nombre: mombre,
            direccion,
            prestamos_efectuados: Vec::new(),
            libros_a_disposicion: HashMap::new(),
            clientes: HashMap::new(),
            fecha,
        };
        let buf = serde_json::to_string(&biblio).unwrap();
        file.write_all(&buf.as_bytes())
            .expect("error escribiendo el archivo");
        biblio
    }

    /// Agrega un cliente nuevo a la colección de clientes.
    pub fn generar_nuevo_cliente(&mut self, cliente: Cliente) {
        self.clientes.insert(cliente.clone().id_cliente, cliente);
        self.escribir_json().expect("error escribiendo el archivo");
    }

    /// Guarda los datos persistentes en un archivo en formato JSON.
    pub fn escribir_json(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(FILE_NAME)?;
        let buf = serde_json::to_string(&self)?;
        file.write_all(&buf.as_bytes())?;
        Ok(())
    }

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
                self.escribir_json().expect("error escribiendo el archivo");
            }
        }
    }

    /// Dado un determinado libro, incrementa la cantidad de copias a disposición.
    pub fn incrementar_copias_a_disposicion(&mut self, libro: Libro) {
        if let Some(copias) = self.libros_a_disposicion.get_mut(&libro) {
            if *copias < u8::max_value() {
                *copias += 1;
                self.escribir_json().expect("error escribiendo el archivo");
            }
        } else {
            self.libros_a_disposicion
                .entry(libro.clone())
                .insert_entry(1);
            self.escribir_json().expect("error escribiendo el archivo");
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
                self.escribir_json().expect("error escribiendo el archivo");
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
            self.escribir_json().expect("error escribiendo el archivo");
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
        Biblioteca::new(
            "Calle 1".to_string(),
            "Biblio".to_string(),
            build_fecha_base(),
        )
    }

    fn build_biblio_con_prestamo() -> (Biblioteca, Cliente) {
        let mut biblio = build_biblio();
        let cliente = build_cliente();
        biblio.prestamos_efectuados = vec![Prestamo {
            libro: build_libro_base(),
            fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
            fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
            cliente: cliente.clone(),
            estado: EstadoPrestamo::Prestamo,
        }];

        biblio.nombre = "B".to_string();
        biblio.direccion = "X".to_string();

        (biblio, cliente)
    }

    fn build_biblio_con_prestamos() -> (Biblioteca, Cliente) {
        let mut biblio = build_biblio();
        let cliente = build_cliente();
        biblio.nombre = "B".to_string();
        biblio.direccion = "X".to_string();

        biblio.prestamos_efectuados = vec![
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            },
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Devuelto,
            },
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            },
        ];

        (biblio, cliente)
    }

    fn build_biblio_con_prestamos_devueltos() -> (Biblioteca, Cliente) {
        let mut biblio = build_biblio();
        let cliente = build_cliente();
        biblio.nombre = "B".to_string();
        biblio.direccion = "X".to_string();
        biblio.prestamos_efectuados = vec![
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            },
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Devuelto,
            },
            Prestamo {
                libro: build_libro_base(),
                fecha_vencimiento: Fecha::new(02, 05, 2025).unwrap(),
                fecha_devolucion: Some(Fecha::new(01, 05, 2025).unwrap()),
                cliente: cliente.clone(),
                estado: EstadoPrestamo::Prestamo,
            },
        ];
        (biblio, cliente)
    }

    fn build_cliente() -> Cliente {
        Cliente::new("Juan".to_string(), 123456, "juan@email.com".to_string())
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
        let (biblio, _cliente) = build_biblio_con_prestamo();
        let expect = biblio.ver_prestamos_a_vencer(15, Fecha::new(01, 05, 2025).unwrap());
        assert_eq!(
            biblio.ver_prestamos_a_vencer(15, Fecha::new(01, 05, 2025).unwrap()),
            expect
        )
    }

    #[test]
    fn contar_prestamos_cliente_correctamente() {
        let (biblio, cliente) = build_biblio_con_prestamo();
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
        let (biblio, _cliente) = build_biblio_con_prestamos();
        let vencidos = biblio.ver_prestamos_vencidos(Fecha::new(02, 05, 2025).unwrap());
        assert_eq!(vencidos.len(), 2);
    }

    #[test]
    fn buscar_prestamo_correctamente() {
        let (biblio, cliente) = build_biblio_con_prestamos();
        let libro = build_libro_base();
        let libro_inexistente = build_libro_02();
        assert!(biblio.buscar_prestamo(cliente.clone(), libro).is_some());
        assert!(biblio.buscar_prestamo(cliente, libro_inexistente).is_none());
    }

    #[test]
    fn devolver_libro_correctamente() {
        let libro = build_libro_base();
        let (mut biblio, cliente) = build_biblio_con_prestamos_devueltos();

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

    #[test]
    fn get_historial_prestamos_correctamente() {
        let (mut biblio, cliente) = build_biblio_con_prestamos_devueltos();
        biblio.generar_nuevo_cliente(cliente.clone());

        let historial_todos =
            biblio.get_historial_prestamos(cliente.id_cliente.clone(), FiltroEstadoPrestamo::Todos);
        assert_eq!(
            historial_todos.len(),
            3,
            "Debe devolver todos los préstamos"
        );

        let historial_prestamo = biblio
            .get_historial_prestamos(cliente.id_cliente.clone(), FiltroEstadoPrestamo::Prestamo);
        assert_eq!(
            historial_prestamo.len(),
            2,
            "Debe devolver solo los préstamos activos"
        );

        let historial_devuelto =
            biblio.get_historial_prestamos(cliente.id_cliente, FiltroEstadoPrestamo::Devuelto);
        assert_eq!(
            historial_devuelto.len(),
            1,
            "Debe devolver solo los préstamos devueltos"
        );
    }
}
