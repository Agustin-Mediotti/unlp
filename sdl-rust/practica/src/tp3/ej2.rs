#[allow(dead_code)]
pub struct Rectangulo {
    pub longitud: u32,
    pub ancho: u32,
}

#[allow(dead_code)]
impl Rectangulo {
    pub fn new(longitud: u32, ancho: u32) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }

    pub fn calcular_area(&self) -> u64 {
        self.longitud as u64 * self.ancho as u64
    }

    pub fn calcular_perimetro(&self) -> u64 {
        (self.longitud as u64 * self.ancho as u64) * 2
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}
