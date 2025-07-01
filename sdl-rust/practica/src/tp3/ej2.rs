pub struct Rectangulo {
    pub longitud: u32,
    pub ancho: u32,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_rectangulo_con_parametros_correctos() {
        let rect = Rectangulo::new(5, 5);

        assert_eq!(rect.ancho, 5);
        assert_eq!(rect.longitud, 5);
    }

    #[test]
    fn calcular_correctamente_area_de_rectangulo() {
        assert_eq!(Rectangulo::new(2, 0).calcular_area(), 0);
        assert_eq!(Rectangulo::new(1, 1).calcular_area(), 1);
        assert_eq!(Rectangulo::new(5, 6).calcular_area(), 30);
    }

    #[test]
    fn calcular_correctamente_perimetro_de_rectangulo() {
        assert_eq!(Rectangulo::new(2, 0).calcular_perimetro(), 0);
        assert_eq!(Rectangulo::new(1, 1).calcular_perimetro(), 2);
        assert_eq!(Rectangulo::new(5, 6).calcular_perimetro(), 60);
    }

    #[test]
    fn identifica_cuadrado_correctamente() {
        assert!(!Rectangulo::new(2, 0).es_cuadrado());
        assert!(Rectangulo::new(1, 1).es_cuadrado());
        assert!(!Rectangulo::new(5, 6).es_cuadrado());
        assert!(Rectangulo::new(u32::max_value(), u32::max_value()).es_cuadrado());
    }
}
