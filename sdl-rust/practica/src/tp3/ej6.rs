#[derive(Debug, PartialEq)]
pub struct Examen {
    pub nombre_materia: String,
    pub nota: u8,
}

#[allow(dead_code)]
pub struct Informe {
    nombre: String,
    numero_ident: u64,
    total_examenes_rendidos: usize,
    promedio_general_notas: f64,
    nota_mas_alta: (u8, String),
    nota_mas_baja: (u8, String),
}

#[derive(Debug, PartialEq)]
pub struct Estudiante {
    pub nombre: String,
    pub numero_ident: u64,
    pub calificaciones: Vec<Examen>,
}

#[allow(dead_code)]
impl Examen {
    pub fn new(nombre_materia: String, nota: u8) -> Self {
        Examen {
            nombre_materia,
            nota,
        }
    }
}

#[allow(dead_code)]
impl Estudiante {
    pub fn new(nombre: String, numero_ident: u64, calificaciones: Vec<Examen>) -> Estudiante {
        Estudiante {
            nombre,
            numero_ident,
            calificaciones,
        }
    }

    pub fn obtener_promedio(&self) -> f64 {
        let total: u32 = self.calificaciones.iter().map(|f| f.nota as u32).sum();
        let cantidad = self.calificaciones.len() as f64;

        if cantidad == 0.0 {
            0.0
        } else {
            total as f64 / cantidad
        }
    }

    pub fn obtener_calificacion_mas_alta(&self) -> u8 {
        self.calificaciones
            .iter()
            .map(|f| f.nota)
            .max()
            .unwrap_or(0)
    }

    pub fn obtener_calificacion_mas_baja(&self) -> u8 {
        self.calificaciones
            .iter()
            .map(|f| f.nota)
            .min()
            .unwrap_or(0)
    }

    pub fn generar_informe(&self) -> Option<Informe> {
        if self.calificaciones.is_empty() {
            None
        } else {
            Some(Informe {
                nombre: self.nombre.clone(),
                numero_ident: self.numero_ident,
                total_examenes_rendidos: self.calificaciones.iter().count(),
                promedio_general_notas: self.calificaciones.iter().count() as f64
                    / self.calificaciones.iter().fold(0, |acc, x| acc + x.nota) as f64,
                nota_mas_alta: (0, "todo".to_owned()),
                nota_mas_baja: (0, "todo".to_owned()),
            })
        }
    }
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
