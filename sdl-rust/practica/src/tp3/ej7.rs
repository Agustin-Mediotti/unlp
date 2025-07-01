#[derive(PartialEq, Debug, Clone)]
pub enum ColorAuto {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub anio: u32,
    pub precio_bruto: f64,
    pub color: ColorAuto,
}

impl Auto {
    pub fn new(
        marca: String,
        modelo: String,
        anio: u32,
        precio_bruto: f64,
        color: ColorAuto,
    ) -> Self {
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
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_max: usize, autos: Vec<Auto>) -> Self {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_max,
            autos,
        }
    }

    pub fn agregar_auto(&mut self, auto: Auto) {
        if self.autos.len() < self.capacidad_max {
            self.autos.push(auto);
        }
    }

    pub fn eliminar_auto(&mut self, auto: Auto) {
        if let Some(pos) = self.autos.iter().position(|f| *f == auto) {
            self.autos.remove(pos);
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
    fn agregar_auto_a_concesionario_correctamente() {
        let mut conce = ConcesionarioAuto::new(
            "Concesionario 44".to_owned(),
            "44 13 y 14".to_owned(),
            5,
            Vec::new(),
        );
        conce.agregar_auto(Auto {
            marca: "BMW".to_owned(),
            modelo: "M3".to_owned(),
            anio: 1995,
            precio_bruto: 30_000.0,
            color: ColorAuto::Rojo,
        });
        assert_eq!(
            conce.autos.len(),
            1,
            "No se agrego el auto al concesionario como se esperaba"
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
        conce.agregar_auto(auto.clone());
        assert_eq!(conce.autos.len(), 1,);
        conce.eliminar_auto(auto);
        assert_eq!(
            conce.autos.len(),
            0,
            "No se elimino el auto al concesionario como se esperaba"
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
        conce.agregar_auto(auto.clone());

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
