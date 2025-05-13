#[derive(Debug, PartialEq)]
pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f32,
    pub numero_ident: u64,
}

#[allow(dead_code)]
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
