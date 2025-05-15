use super::ej3::Fecha;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico_final: String,
    pub tratamiento: String,
    pub fecha_proxima_visita: Option<Fecha>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Propietario {
    pub nombre: String,
    pub direccion: String,
    pub telefono: u32,
}

#[derive(Debug, PartialEq)]
pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: u32,
    pub registro_atenciones: Vec<Atencion>,
    pub cola_atencion: VecDeque<Mascota>,
    pub mascota_actual: Option<Mascota>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u8,
    pub tipo_animal: TipoAnimal,
    pub propietario: Propietario,
}

#[allow(dead_code)]
impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Self {
        Veterinaria {
            nombre,
            direccion,
            id,
            registro_atenciones: Vec::<Atencion>::new(),
            cola_atencion: VecDeque::<Mascota>::new(),
            mascota_actual: None,
        }
    }

    pub fn agregar_a_cola_de_atencion(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_a_cola_de_atencion_prioridad(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    pub fn atender_siguiente_mascota(&mut self) {
        self.mascota_actual = self.cola_atencion.pop_front();
    }

    pub fn eliminar_mascota_de_cola(&mut self, mascota: Mascota) {
        if let Some(pos) = self.cola_atencion.iter().position(|f| *f == mascota) {
            self.cola_atencion.remove(pos);
        }
    }

    pub fn registrar_atencion(
        &mut self,
        diagnostico_final: String,
        tratamiento: String,
        fecha_proxima_visita: Option<Fecha>,
    ) {
        if let Some(mascota) = &self.mascota_actual {
            self.registro_atenciones.push(Atencion {
                mascota: mascota.to_owned(),
                diagnostico_final,
                tratamiento,
                fecha_proxima_visita,
            });
        }
    }

    pub fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_propietario: String,
        telefono: u32,
    ) -> Option<Atencion> {
        if let Some(pos) = self.registro_atenciones.iter().position(|f| {
            *f.mascota.nombre == nombre_mascota
                && *f.mascota.propietario.nombre == nombre_propietario
                && f.mascota.propietario.telefono == telefono
        }) {
            Some(self.registro_atenciones[pos].to_owned())
        } else {
            None
        }
    }

    pub fn modificar_diagnostico(&mut self, indice_atencion: usize, diagnostico_final: String) {
        if let Some(atencion) = self.registro_atenciones.get_mut(indice_atencion) {
            atencion.diagnostico_final = diagnostico_final;
        }
    }

    pub fn modificar_fecha_proxima_visita(&mut self, indice_atencion: usize, fecha: Option<Fecha>) {
        if let Some(atencion) = self.registro_atenciones.get_mut(indice_atencion) {
            atencion.fecha_proxima_visita = fecha;
        }
    }

    pub fn eliminar_atencion(&mut self, indice_atencion: usize) {
        if self.registro_atenciones.get(indice_atencion).is_some() {
            self.registro_atenciones.remove(indice_atencion);
        }
    }
}
