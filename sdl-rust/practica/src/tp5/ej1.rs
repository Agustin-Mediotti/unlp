use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::Write;

const FILE_NAME: &str = "src/tp5/archivo_autos.json";

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ColorAuto {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub anio: u32,
    pub precio_bruto: f64,
    pub color: ColorAuto,
}

impl Auto {
    fn new(marca: String, modelo: String, anio: u32, precio_bruto: f64, color: ColorAuto) -> Self {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut total: f64 = self.precio_bruto;
        match self.color {
            ColorAuto::Rojo | ColorAuto::Verde | ColorAuto::Amarillo => {
                total += self.precio_bruto * 25.0 / 100.0
            }
            _ => total -= self.precio_bruto * 10.0 / 100.0,
        };
        if self.marca == "BMW" {
            total += self.precio_bruto * 15.0 / 100.0;
        }
        if self.anio < 2000 {
            total -= self.precio_bruto * 5.0 / 100.0;
        }
        total
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorConcecionaria {
    ErrorCapacidadMaxima(String),
    ErrorConcecionarioVacio,
    ErrorAutoNoEncontrado,
    ErrorEscritura(String),
}

impl From<std::io::Error> for ErrorConcecionaria {
    fn from(e: std::io::Error) -> Self {
        ErrorConcecionaria::ErrorEscritura(e.to_string())
    }
}

impl Display for ErrorConcecionaria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorConcecionaria::ErrorCapacidadMaxima(e) => write!(f, "{e}"),
            ErrorConcecionaria::ErrorConcecionarioVacio => {
                write!(f, "No se puede eliminar un auto de una concecionaria vacía")
            }
            ErrorConcecionaria::ErrorAutoNoEncontrado => {
                write!(f, "El auto no se encontró en la concecionaria")
            }
            ErrorConcecionaria::ErrorEscritura(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_max: usize, autos: Vec<Auto>) -> Self {
        let mut file = File::create(FILE_NAME).unwrap();
        let concecionaria = ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_max,
            autos,
        };
        let buf = serde_json::to_string(&concecionaria).unwrap();
        file.write_all(&buf.as_bytes())
            .expect("error escribiendo el archivo");
        concecionaria
    }

    pub fn escribir_json(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(FILE_NAME)?;
        let buf = serde_json::to_string(&self)?;
        file.write_all(&buf.as_bytes())?;
        Ok(())
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> Result<(), ErrorConcecionaria> {
        if self.autos.len() < self.capacidad_max {
            self.autos.push(auto);
            self.escribir_json()?;
            Ok(())
        } else {
            Err(ErrorConcecionaria::ErrorCapacidadMaxima(
                self.capacidad_max.to_string(),
            ))
        }
    }

    pub fn eliminar_auto(&mut self, auto: Auto) -> Result<(), ErrorConcecionaria> {
        if self.autos.is_empty() {
            return Err(ErrorConcecionaria::ErrorConcecionarioVacio);
        }
        if let Some(pos) = self.autos.iter().position(|f| *f == auto) {
            self.autos.remove(pos);
            self.escribir_json()?;
            return Ok(());
        } else {
            Err(ErrorConcecionaria::ErrorAutoNoEncontrado)
        }
    }

    pub fn buscar_auto(&self, auto: Auto) -> Option<Auto> {
        if let Some(pos) = self.autos.iter().position(|f| *f == auto) {
            Some(self.autos[pos].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_nuevo_concesionario_auto_correctamente() {
        assert_eq!(
            ConcesionarioAuto::new(
                "Concesionario 44".to_owned(),
                "44 13 y 14".to_owned(),
                5,
                Vec::new()
            ),
            ConcesionarioAuto {
                nombre: "Concesionario 44".to_owned(),
                direccion: "44 13 y 14".to_owned(),
                capacidad_max: 5,
                autos: Vec::new()
            },
            "No se creo el objeto como se esperaba"
        );
    }

    #[test]
    fn crea_auto_nuevo_correctamente() {
        let auto = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2010,
            15000.0,
            ColorAuto::Negro,
        );

        assert_eq!(auto.marca, "Toyota");
        assert_eq!(auto.modelo, "Corolla");
        assert_eq!(auto.anio, 2010);
        assert_eq!(auto.precio_bruto, 15000.0);
        assert_eq!(auto.color, ColorAuto::Negro);
    }

    #[test]
    fn calcular_precio_auto_color_azul_sin_descuento_marca() {
        let auto = Auto {
            marca: "Toyota".to_owned(),
            modelo: "Corolla".to_owned(),
            anio: 2020,
            precio_bruto: 20000.0,
            color: ColorAuto::Azul,
        };

        assert_eq!(auto.calcular_precio(), 18000.0);
    }

    #[test]
    fn agregar_auto_a_concesionario_correctamente() {
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );

        conce
            .agregar_auto(Auto {
                marca: "BMW".to_owned(),
                modelo: "M3".to_owned(),
                anio: 1995,
                precio_bruto: 30_000.0,
                color: ColorAuto::Rojo,
            })
            .unwrap();

        assert_eq!(
            conce.autos.len(),
            1,
            "No se agrego el auto al concesionario como se esperaba"
        );
    }

    #[test]
    fn agregar_auto_concecionaria_llena() {
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            1,
            Vec::new(),
        );

        conce
            .agregar_auto(Auto {
                marca: "BMW".to_owned(),
                modelo: "M3".to_owned(),
                anio: 1995,
                precio_bruto: 30_000.0,
                color: ColorAuto::Rojo,
            })
            .unwrap();

        assert_eq!(
            conce.agregar_auto(Auto {
                marca: "BMW".to_owned(),
                modelo: "M3".to_owned(),
                anio: 1995,
                precio_bruto: 30_000.0,
                color: ColorAuto::Rojo,
            }),
            Err(ErrorConcecionaria::ErrorCapacidadMaxima("1".to_string())),
            "Se esperaba un error de capacidad 1"
        );
    }

    #[test]
    fn eliminar_auto_de_concesionario_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(auto.clone()).unwrap();
        assert_eq!(conce.autos.len(), 1,);
        conce.eliminar_auto(auto).unwrap();
        assert_eq!(
            conce.autos.len(),
            0,
            "No se elimino el auto al concesionario como se esperaba"
        );
    }

    #[test]
    fn eliminar_auto_no_existente_devuelve_error() {
        let auto = Auto::new(
            "Ford".to_string(),
            "Fiesta".to_string(),
            2015,
            10000.0,
            ColorAuto::Blanco,
        );

        let otro_auto = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2020,
            20000.0,
            ColorAuto::Azul,
        );

        let mut conce = ConcesionarioAuto::new(
            "Concesionario Test".to_string(),
            "Calle Falsa 123".to_string(),
            2,
            vec![otro_auto],
        );

        assert_eq!(
            conce.eliminar_auto(auto),
            Err(ErrorConcecionaria::ErrorAutoNoEncontrado),
            "Se esperaba error al intentar eliminar un auto inexistente"
        );
    }

    #[test]
    fn eliminar_auto_de_concesionario_vacio() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            1,
            Vec::new(),
        );
        assert_eq!(
            conce.eliminar_auto(auto),
            Err(ErrorConcecionaria::ErrorConcecionarioVacio),
            "Se esperaba un error al eliminar un auto de una concecionaria vacia"
        );
    }

    #[test]
    fn buscar_auto_en_concesionario_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(auto.clone()).unwrap();

        assert_eq!(
            conce.buscar_auto(auto.clone()),
            Some(auto),
            "No se encontro el auto en el concesionario como se esperaba"
        );
    }

    #[test]
    fn calcular_precio_de_auto_correctamente() {
        let auto = Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        };

        assert_eq!(
            auto.calcular_precio(),
            40_500.0,
            "No se calculo el precio del auto como se esperaba"
        );
    }
}
