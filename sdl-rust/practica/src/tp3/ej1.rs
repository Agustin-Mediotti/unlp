pub struct Persona {
    pub nombre: String,
    pub edad: u8,
    pub dir: Option<String>,
}

impl Persona {
    pub fn new(nombre: String, edad: u8, dir: Option<String>) -> Persona {
        Persona { nombre, edad, dir }
    }

    pub fn to_string(&self) -> String {
        let direccion = match &self.dir {
            Some(dir) => dir.clone(),
            None => "Sin dirección".to_string(),
        };

        format!("{}, {}, {}", self.nombre, self.edad, direccion)
    }

    pub fn obtener_edad(&self) -> u8 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nue_dir: String) {
        self.dir = Some(nue_dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_nueva_persona_sin_direccion() {
        let persona = Persona::new("Fausto".to_string(), 11, None);
        assert_eq!(persona.nombre, "Fausto");
        assert_eq!(persona.edad, 11);
        assert_eq!(persona.dir, None);
    }

    #[test]
    fn crea_nueva_persona_con_direccion() {
        let persona = Persona::new(
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
        let persona_to_string = Persona::new(
            "Fausto".to_string(),
            11,
            Some("Calle Falsa 123".to_string()),
        )
        .to_string();

        assert_eq!(persona_to_string, "Fausto, 11, Calle Falsa 123")
    }

    #[test]
    fn obtener_edad_de_persona() {
        let persona = Persona::new(
            "Fausto".to_string(),
            11,
            Some("Calle Falsa 123".to_string()),
        );

        assert_eq!(persona.obtener_edad(), 11);
    }

    #[test]
    fn actualizar_direccion_de_persona() {
        let mut persona = Persona::new("Fausto".to_string(), 11, None);
        persona.actualizar_direccion("Fausto, 11, Calle Falsa 123".to_string());

        assert_eq!(persona.dir, Some("Fausto, 11, Calle Falsa 123".to_string()));
    }
}
