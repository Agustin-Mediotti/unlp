use super::ej3::Fecha;
use std::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_nueva_veterinaria_correctamente() {
        let vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        assert_eq!(
            vete,
            Veterinaria {
                nombre: "Vet01".to_owned(),
                direccion: "44 n552".to_owned(),
                id: 32,
                registro_atenciones: Vec::<Atencion>::new(),
                cola_atencion: VecDeque::<Mascota>::new(),
                mascota_actual: None,
            },
            "No se creo el objeto veterinaria como se esperaba"
        );
    }

    #[test]
    fn agregar_a_cola_de_atencion_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        assert!(
            vete.cola_atencion.contains(&mascota),
            "No se agrego la mascota a la cola de atencion como se esperaba"
        );
    }

    #[test]
    fn agregar_a_cola_de_atencion_prioridad_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        assert!(
            vete.cola_atencion[0] == mascota_2,
            "No se agrego la mascota a la cola de atencion con prioridad como se esperaba"
        );
    }

    #[test]
    fn atender_siguiente_mascota_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();

        assert!(
            vete.cola_atencion[0] == mascota.clone(),
            "No se quito la mascota a la cola de atencion como se esperaba"
        );
        assert!(
            vete.mascota_actual == Some(mascota_2),
            "No se agrego la mascota como mascota actual como se esperaba"
        );
    }

    #[test]
    fn eliminar_mascota_de_cola_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.eliminar_mascota_de_cola(mascota);
        assert!(
            vete.cola_atencion.len() == 1,
            "No se elimino la mascota de la cola de atencion como se esperaba"
        );
    }

    #[test]
    fn registrar_atencion_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();
        vete.registrar_atencion(
            "Mala alimentacion".to_owned(),
            "Mejor alimento".to_owned(),
            Fecha::new(20, 06, 2025),
        );
        assert!(
            vete.registro_atenciones.len() == 1,
            "No se registro una nueva atencion como se esperaba"
        );
    }

    #[test]
    fn buscar_atencion_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();
        vete.registrar_atencion(
            "Mala alimentacion".to_owned(),
            "Mejor alimento".to_owned(),
            Fecha::new(20, 06, 2025),
        );
        assert!(
            vete.buscar_atencion("Picudo".to_owned(), "Pedro".to_owned(), 726123)
                .is_some(),
            "No se encontro la atencion guardada en el registro como se esperaba"
        );
    }

    #[test]
    fn modificar_diagnostico_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();
        vete.registrar_atencion(
            "Mala alimentacion".to_owned(),
            "Mejor alimento".to_owned(),
            Fecha::new(20, 06, 2025),
        );
        vete.modificar_diagnostico(0, "Mal aliento".to_owned());
        assert!(
            vete.buscar_atencion("Picudo".to_owned(), "Pedro".to_owned(), 726123)
                .unwrap()
                .diagnostico_final
                == "Mal aliento".to_owned(),
            "No se modifico el diagnostico en la atencion registrada como se esperaba"
        );
    }

    #[test]
    fn modificar_fecha_proxima_visita_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();
        vete.registrar_atencion(
            "Mala alimentacion".to_owned(),
            "Mejor alimento".to_owned(),
            Fecha::new(20, 06, 2025),
        );
        vete.modificar_fecha_proxima_visita(0, Fecha::new(20, 05, 2025));
        assert!(
            vete.buscar_atencion("Picudo".to_owned(), "Pedro".to_owned(), 726123)
                .unwrap()
                .fecha_proxima_visita
                == Fecha::new(20, 05, 2025),
            "No se modifico la fecha de proxima visita en la atencion registrada como se esperaba"
        );
    }

    #[test]
    fn eliminar_atencion_correctamente() {
        let mut vete = Veterinaria::new("Vet01".to_owned(), "44 n552".to_owned(), 32);
        let mascota = Mascota {
            nombre: "Rodolfo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Perro,
            propietario: Propietario {
                nombre: "Fabian".to_owned(),
                direccion: "127bis y 22".to_owned(),
                telefono: 6271254,
            },
        };
        let mascota_2 = Mascota {
            nombre: "Picudo".to_owned(),
            edad: 15,
            tipo_animal: TipoAnimal::Gato,
            propietario: Propietario {
                nombre: "Pedro".to_owned(),
                direccion: "17 y 52".to_owned(),
                telefono: 726123,
            },
        };
        vete.agregar_a_cola_de_atencion(mascota.clone());
        vete.agregar_a_cola_de_atencion_prioridad(mascota_2.clone());
        vete.atender_siguiente_mascota();
        vete.registrar_atencion(
            "Mala alimentacion".to_owned(),
            "Mejor alimento".to_owned(),
            Fecha::new(20, 06, 2025),
        );
        vete.eliminar_atencion(0);
        assert!(
            vete.registro_atenciones.len() == 0,
            "No se elimino la atencion registrada como se esperaba"
        );
    }
}
