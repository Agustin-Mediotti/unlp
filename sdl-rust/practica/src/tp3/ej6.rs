#[derive(Debug, PartialEq)]
pub struct Examen {
    pub nombre_materia: String,
    pub nota: u8,
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
}
