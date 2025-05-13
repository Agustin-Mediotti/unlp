#[derive(Debug, PartialEq)]
pub enum TrianguloTipo {
    Equilatero,
    Isosceles,
    Escaleno,
}

#[derive(Debug, PartialEq)]
pub struct Triangulo {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[allow(dead_code)]
impl Triangulo {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Triangulo { x, y, z }
    }

    pub fn determinar_tipo(&self) -> Option<TrianguloTipo> {
        if self.x == self.y && self.y == self.z {
            Some(TrianguloTipo::Equilatero)
        } else if (self.x == self.y) || (self.x == self.z) || (self.z == self.y) {
            Some(TrianguloTipo::Isosceles)
        } else if (self.x != self.y) && (self.x != self.z) && (self.z != self.y) {
            Some(TrianguloTipo::Escaleno)
        } else {
            None
        }
    }

    pub fn calcular_area(&self) -> f64 {
        (f64::sqrt(3.0) / 4.0) * (self.y * self.y) as f64
    }

    pub fn calcular_perimetro(&self) -> i64 {
        self.x + self.y + self.z
    }
}
