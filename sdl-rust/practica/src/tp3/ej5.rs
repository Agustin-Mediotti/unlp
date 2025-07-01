#[derive(Debug, PartialEq)]
pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f32,
    pub numero_ident: u64,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f32, numero_ident: u64) -> Self {
        Producto {
            nombre,
            precio_bruto,
            numero_ident,
        }
    }

    pub fn calcular_impuestos(&self, porcentaje_de_impuestos: f32) -> f32 {
        self.precio_bruto * (porcentaje_de_impuestos / 100.0)
    }

    pub fn aplicar_descuento(&self, porcentaje_de_descuento: f32) -> f32 {
        self.precio_bruto * (porcentaje_de_descuento / 100.0)
    }

    pub fn calcular_precio_total(
        &self,
        porcentaje_de_impuestos: Option<f32>,
        porcentaje_de_descuento: Option<f32>,
    ) -> f32 {
        if let Some(descuento) = porcentaje_de_descuento {
            if let Some(impuesto) = porcentaje_de_impuestos {
                return self.precio_bruto + self.calcular_impuestos(impuesto)
                    - self.aplicar_descuento(descuento);
            } else {
                return self.precio_bruto - self.aplicar_descuento(descuento);
            }
        } else if let Some(impuesto) = porcentaje_de_impuestos {
            return self.precio_bruto + self.calcular_impuestos(impuesto);
        } else {
            self.precio_bruto
        }

        /*
           Lo mismo pero mas aburrido ._.

           let mut total = self.precio_bruto;
           if porcentaje_de_descuento.is_some() {
               total = -self.aplicar_descuento(porcentaje_de_descuento.unwrap());
           }
           if porcentaje_de_impuestos.is_some() {
               total += self.calcular_impuestos(porcentaje_de_impuestos.unwrap())
           }
           total
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_nuevo_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192),
            Producto {
                nombre: "Nodemcu Esp32".to_string(),
                precio_bruto: 14990.00,
                numero_ident: 3032192
            }
        );
    }

    #[test]
    fn calcula_impuestos_de_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192).calcular_impuestos(20.00),
            14990.00 * (20.00 / 100.0)
        );
    }

    #[test]
    fn calcula_descuento_de_producto_correctamente() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192).aplicar_descuento(35.00),
            14990.00 * (35.00 / 100.0)
        );
    }

    #[test]
    fn calcula_correctamente_precio_total_de_producto() {
        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(Some(20.00), None),
            14990.00 + (14990.00 * (20.00 / 100.0)),
            "El precio total no se calculo como se esperaba (20% impuestos y 0% descuento)"
        );

        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(None, Some(35.00)),
            14990.00 - (14990.00 * (35.00 / 100.0)),
            "El precio total no se calculo como se esperaba (0% impuestos y 35% descuento)"
        );

        assert_eq!(
            Producto::new("Nodemcu Esp32".to_string(), 14990.00, 3032192)
                .calcular_precio_total(Some(20.00), Some(35.00)),
            14990.00 - (14990.00 * (35.00 / 100.0)) + (14990.00 * (20.00 / 100.0)),
            "El precio total no se calculo como se esperaba (20% impuestos y 35% descuento)"
        );
    }
}
