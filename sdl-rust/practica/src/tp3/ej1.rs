pub struct Persona {
    pub nombre: String,
    pub edad: u8,
    pub dir: Option<String>,
}

#[allow(dead_code)]
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
