#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

pub struct Playlist {
    pub canciones: Vec<Cancion>,
    pub nombre: String,
}

#[allow(dead_code)]
impl Playlist {
    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, cancion: Cancion) {
        if let Some(pos) = self.canciones.iter().position(|f| *f == cancion) {
            self.canciones.remove(pos);
        }
    }

    pub fn mover_cancion(&mut self, cancion: Cancion, posicion: usize) {
        if let Some(pos) = self.canciones.iter().position(|f| *f == cancion) {
            let cancion = self.canciones.remove(pos);
            let nueva_posicion = if posicion > self.canciones.len() {
                self.canciones.len()
            } else {
                posicion
            };

            self.canciones.insert(nueva_posicion, cancion);
        }
    }

    pub fn buscar_cancion_por_nombre(&self, cancion: Cancion) -> Option<&Cancion> {
        self.canciones.iter().find(|f| **f == cancion)
    }

    pub fn obtener_canciones_por_genero(&self, genero: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|f| f.genero == genero)
            .collect()
    }

    pub fn obtener_canciones_por_artista(&self, artista: String) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|f| f.artista == artista)
            .collect()
    }

    pub fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    pub fn limpiar_playlist(&mut self) {
        self.canciones.clear();
    }
}
