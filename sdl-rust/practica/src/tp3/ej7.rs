#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum ColorAuto {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub anio: u32,
    pub precio_bruto: f64,
    pub color: ColorAuto,
}

#[allow(dead_code)]
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

    #[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_max: usize,
    pub autos: Vec<Auto>,
}

#[allow(dead_code)]
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
