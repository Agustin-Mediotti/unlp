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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agrega_cancion_a_playlist_correctamente() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());

        assert_eq!(
            playlist.canciones[0], cancion,
            "No se agrego la cancion a la playlist como se esperaba"
        );
    }

    #[test]
    fn elimina_cancion_de_playlist_correctamente() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());
        playlist.eliminar_cancion(cancion);

        assert_eq!(
            playlist.canciones.len(),
            0,
            "No se elimino la cancion de la playlist como se esperaba"
        );
    }

    #[test]
    fn mueve_cancion_a_una_determinada_posicion_de_la_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2);

        playlist.mover_cancion(cancion_1.clone(), 1);

        assert_eq!(
            playlist.canciones[1], cancion_1,
            "No se movio la cancion de la playlist a la posicion que se esperaba"
        );
    }

    #[test]
    fn busca_cancion_por_nombre_en_playlist() {
        let cancion = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion.clone());

        assert_eq!(
            playlist
                .buscar_cancion_por_nombre(cancion.clone())
                .unwrap()
                .to_owned(),
            cancion,
            "No se encontro la cancion en la playlist como se esperaba"
        );
    }

    #[test]
    fn obtener_canciones_por_genero_en_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        let mut expected = Vec::<&Cancion>::new();
        expected.push(&cancion_1);
        expected.push(&cancion_2);

        assert_eq!(
            playlist.obtener_canciones_por_genero(Genero::Otros),
            expected,
            "No se obtuvieron las canciones de la playlist por genero como se esperaba"
        );
    }

    #[test]
    fn obtener_canciones_por_artista_en_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        let mut expected = Vec::<&Cancion>::new();
        expected.push(&cancion_2);

        assert_eq!(
            playlist.obtener_canciones_por_artista("Lacuna Coil".to_owned()),
            expected,
            "No se obtuvieron las canciones de la playlist por artista como se esperaba"
        );
    }

    #[test]
    fn modificar_titulo_de_playlist() {
        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.modificar_titulo("Metal only".to_owned());

        assert_eq!(
            playlist.nombre,
            "Metal only".to_owned(),
            "No se cambio el título de la playlist como se esperaba"
        );
    }

    #[test]
    fn eliminar_canciones_de_playlist() {
        let cancion_1 = Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        };
        let cancion_2 = Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        };

        let mut playlist = Playlist {
            nombre: "Mi Playlist".to_owned(),
            canciones: Vec::new(),
        };
        playlist.agregar_cancion(cancion_1.clone());
        playlist.agregar_cancion(cancion_2.clone());

        playlist.limpiar_playlist();

        assert_eq!(
            playlist.canciones,
            Vec::<Cancion>::new(),
            "No se eliminaron todas las canciones de la playlist como se esperaba"
        );
    }
}
