use serde::Serialize;
use std::fs::File;
use std::io::Write;

const FILE_NAME: &str = "src/tp5/archivo_canciones.json";

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

#[derive(Serialize)]
pub struct Playlist {
    pub canciones: Vec<Cancion>,
    pub nombre: String,
}

#[derive(Debug)]
pub enum PlaylistError {
    JsonFileError(std::io::Error),
    SongNotFound,
}

impl From<std::io::Error> for PlaylistError {
    fn from(e: std::io::Error) -> Self {
        PlaylistError::JsonFileError(e)
    }
}

impl Playlist {
    pub fn new(nombre: String) -> Self {
        let mut file = File::create(FILE_NAME).unwrap();
        let playlist = Playlist {
            canciones: Vec::new(),
            nombre,
        };
        let buf = serde_json::to_string(&playlist).unwrap();
        file.write_all(&buf.as_bytes())
            .expect("error escribiendo el archivo");
        playlist
    }

    pub fn escribir_json(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(FILE_NAME)?;
        let buf = serde_json::to_string(&self)?;
        file.write_all(&buf.as_bytes())?;
        Ok(())
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) -> Result<(), PlaylistError> {
        self.canciones.push(cancion);
        self.escribir_json()?;
        Ok(())
    }

    pub fn eliminar_cancion(&mut self, cancion: Cancion) -> Result<(), PlaylistError> {
        if let Some(pos) = self.canciones.iter().position(|f| *f == cancion) {
            self.canciones.remove(pos);
            self.escribir_json()?;
            Ok(())
        } else {
            Err(PlaylistError::SongNotFound)
        }
    }

    pub fn mover_cancion(
        &mut self,
        cancion: Cancion,
        posicion: usize,
    ) -> Result<(), PlaylistError> {
        if let Some(pos) = self.canciones.iter().position(|f| *f == cancion) {
            let cancion = self.canciones.remove(pos);

            let nueva_posicion = if posicion > self.canciones.len() {
                self.canciones.len()
            } else {
                posicion // Si la posicion está fuera de rango, la nueva posicion sera la última del vector.
            };

            self.canciones.insert(nueva_posicion, cancion);
            self.escribir_json()?;
            Ok(())
        } else {
            Err(PlaylistError::SongNotFound)
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

    fn build_cancion_01() -> Cancion {
        Cancion {
            titulo: "Flying Whales".to_owned(),
            genero: Genero::Otros,
            artista: "Gojira".to_owned(),
        }
    }

    fn build_cancion_02() -> Cancion {
        Cancion {
            titulo: "Trance Awake".to_owned(),
            genero: Genero::Otros,
            artista: "Lacuna Coil".to_owned(),
        }
    }

    fn build_cancion_from(titulo: String, artista: String) -> Cancion {
        Cancion {
            titulo,
            genero: Genero::Otros,
            artista,
        }
    }

    fn build_set_canciones() -> (Cancion, Cancion) {
        (build_cancion_01(), build_cancion_02())
    }

    fn build_playlist() -> Playlist {
        Playlist::new("Mi Playlist".to_owned())
    }

    #[test]
    fn agrega_cancion_a_playlist_correctamente() {
        let mut playlist = build_playlist();
        let cancion = build_cancion_01();

        playlist.agregar_cancion(cancion.clone()).unwrap();

        assert_eq!(
            playlist.canciones[0], cancion,
            "No se agrego la cancion a la playlist como se esperaba"
        );
    }

    #[test]
    fn elimina_cancion_de_playlist_correctamente() {
        let mut playlist = build_playlist();
        let cancion = build_cancion_01();

        playlist.agregar_cancion(cancion.clone()).unwrap();
        playlist.eliminar_cancion(cancion).unwrap();

        assert_eq!(
            playlist.canciones.len(),
            0,
            "No se elimino la cancion de la playlist como se esperaba"
        );
    }

    #[test]
    fn mueve_cancion_a_una_determinada_posicion_de_la_playlist() {
        let mut playlist = build_playlist();

        let (cancion_1, cancion_2) = build_set_canciones();

        playlist.agregar_cancion(cancion_1.clone()).unwrap();
        playlist.agregar_cancion(cancion_2).unwrap();

        playlist.mover_cancion(cancion_1.clone(), 1).unwrap();

        assert_eq!(
            playlist.canciones[1], cancion_1,
            "No se movio la cancion de la playlist a la posicion que se esperaba"
        );
    }

    #[test]
    fn mueve_cancion_a_una_posicion_fuera_de_rango_de_la_playlist() {
        let mut playlist = build_playlist();

        let (cancion_1, cancion_2) = build_set_canciones();

        playlist.agregar_cancion(cancion_1.clone()).unwrap();
        playlist.agregar_cancion(cancion_2).unwrap();

        playlist.mover_cancion(cancion_1.clone(), 3).unwrap();

        assert_eq!(
            playlist.canciones[1], cancion_1,
            "Se esperaba que la cancion de la playlist se mueva a la última posicion del vector"
        );
    }

    #[test]
    fn busca_cancion_por_nombre_en_playlist() {
        let mut playlist = build_playlist();
        let cancion = build_cancion_01();

        playlist.agregar_cancion(cancion.clone()).unwrap();

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
        let mut playlist = build_playlist();
        let (cancion_1, cancion_2) = build_set_canciones();

        playlist.agregar_cancion(cancion_1.clone()).unwrap();
        playlist.agregar_cancion(cancion_2.clone()).unwrap();

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
        let mut playlist = build_playlist();
        let (cancion_1, cancion_2) = build_set_canciones();

        playlist.agregar_cancion(cancion_1.clone()).unwrap();
        playlist.agregar_cancion(cancion_2.clone()).unwrap();

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
        let mut playlist = build_playlist();
        playlist.modificar_titulo("Metal only".to_owned());

        assert_eq!(
            playlist.nombre,
            "Metal only".to_owned(),
            "No se cambio el título de la playlist como se esperaba"
        );
    }

    #[test]
    fn eliminar_canciones_de_playlist() {
        let mut playlist = build_playlist();
        let (cancion_1, cancion_2) = build_set_canciones();

        playlist.agregar_cancion(cancion_1.clone()).unwrap();
        playlist.agregar_cancion(cancion_2.clone()).unwrap();

        playlist.limpiar_playlist();

        assert_eq!(
            playlist.canciones,
            Vec::<Cancion>::new(),
            "No se eliminaron todas las canciones de la playlist como se esperaba"
        );
    }
}
