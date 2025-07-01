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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_nuevo_triangulo_correctamente() {
        assert_eq!(Triangulo::new(5, 5, 5), Triangulo { x: 5, y: 5, z: 5 });
        assert_ne!(Triangulo::new(2, 7, 3), Triangulo { x: 1, y: 1, z: 1 });
    }

    #[test]
    fn determinar_tipo_triangulo_correctamente() {
        assert_eq!(
            Triangulo::new(5, 5, 5).determinar_tipo().unwrap(),
            TrianguloTipo::Equilatero
        );
        assert_eq!(
            Triangulo::new(2, 7, 1).determinar_tipo().unwrap(),
            TrianguloTipo::Escaleno
        );

        assert_eq!(
            Triangulo::new(2, 2, 1).determinar_tipo().unwrap(),
            TrianguloTipo::Isosceles
        );
    }

    #[test]
    fn calcula_correctamente_el_area_del_rectangulo() {
        assert_eq!(
            Triangulo::new(5, 5, 5).calcular_area(),
            (f64::sqrt(3.0) / 4.0) * (5.0_f64.powf(2.0))
        );
        assert_eq!(
            Triangulo::new(7, 2, 6).calcular_area(),
            (f64::sqrt(3.0) / 4.0) * (2.0_f64.powf(2.0))
        );
    }

    #[test]
    fn calcular_correctamente_perimetro_del_triangulo() {
        assert_eq!(Triangulo::new(5, 5, 5).calcular_perimetro(), 15);
        assert_eq!(Triangulo::new(2, 7, 9).calcular_perimetro(), 18);
        assert_eq!(Triangulo::new(2, 3, 5).calcular_perimetro(), 10);
    }
}
