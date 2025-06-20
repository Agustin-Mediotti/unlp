use std::collections::HashMap;
use time::Date;

enum MedioDePago {
    TarjetaCredito,
    TarjetaDebito,
    Transferencia,
    Efectivo,
}

struct DatosPersonales {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}

struct Vendedor {
    datos_personales: DatosPersonales,
    legajo: String,
    antiguedad: u32,
    salario: f64,
}

struct Cliente {
    datos_personales: DatosPersonales,
    subscripcion: Option<String>,
}

impl Cliente {
    pub fn esta_subscripto(&self) -> bool {
        self.subscripcion != None
    }
}

struct Producto {
    nombre: String,
    categoria: String,
    precio_base: f64,
}

struct Venta {
    fecha: Date,
    productos: Vec<(Producto, u32)>,
    vendedor: Vendedor,
    cliente: Cliente,
    medio_de_pago: MedioDePago,
}

impl Venta {
    fn calcular_precio_final(
        &self,
        descuento_newsletter: f64,
        categorias: HashMap<String, f64>,
    ) -> f64 {
        let mut precio_final: f64 = 0.0;
        self.productos.iter().for_each(|f| {
            if let Some(descuento) = categorias.get(&f.0.categoria) {
                precio_final += (f.0.precio_base * (1.0 - (descuento / 100.00))) * (f.1 as f64); // precio base - descuento de categoria (si aplica)
            } else {
                precio_final += f.0.precio_base * (f.1 as f64);
            };

            if self.cliente.esta_subscripto() {
                precio_final = precio_final * (1.0 - (descuento_newsletter / 100.00)); // precio parcial - descuento por newsletter (si aplica)
            };
        });
        precio_final
    }
}

struct App {
    ventas: Vec<Venta>,
    categorias: HashMap<String, u32>, // nombre de categoria y % de descuento
    descuento_newsletter: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            ventas: Vec::new(),
            categorias: HashMap::new(),
            descuento_newsletter: 0,
        }
    }

    pub fn agregar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }

    pub fn generar_reporte_categorias(&self) -> String {
        if self.ventas.is_empty() {
            return "No hay ventas registradas.".to_owned();
        }

        let mut totales_por_categoria: HashMap<String, u32> = HashMap::new();
        for venta in &self.ventas {
            for (producto, cant) in &venta.productos {
                totales_por_categoria
                    .entry(producto.categoria.clone())
                    .and_modify(|c| *c += *cant)
                    .or_insert(*cant);
            }
        }

        let mut categorias_ordenadas: Vec<(&String, &u32)> = totales_por_categoria.iter().collect();
        categorias_ordenadas.sort_by_key(|(cat, _)| cat.to_owned());

        let mut report: String = String::new();
        for (categoria, cant) in categorias_ordenadas {
            report.push_str(&format!("{}: {}; ", categoria, cant));
        }
        report.trim().to_owned()
    }

    pub fn generar_reporte_vendedor(&self) -> String {
        if self.ventas.is_empty() {
            return "No hay ventas registradas.".to_owned();
        }
        let mut totales_por_vendedor: HashMap<String, u32> = HashMap::new();
        for venta in &self.ventas {
            totales_por_vendedor
                .entry(venta.vendedor.legajo.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut legajos_ordenados: Vec<(&String, &u32)> = totales_por_vendedor.iter().collect();
        legajos_ordenados.sort_by_key(|(cat, _)| cat.to_owned());

        let mut report = String::new();
        for (legajo, cantidad) in legajos_ordenados {
            report.push_str(&format!("{}: {}; ", legajo, cantidad));
        }
        report.trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_fecha() -> Date {
        Date::from_julian_day(2_458_485).expect("deberia devolver una fecha")
    }

    fn build_datos_personales() -> DatosPersonales {
        DatosPersonales {
            nombre: "name".to_owned(),
            apellido: "lastname".to_owned(),
            direccion: "calle falsa 123".to_owned(),
            dni: "12345678910".to_owned(),
        }
    }

    fn build_vendendor() -> Vendedor {
        Vendedor {
            datos_personales: build_datos_personales(),
            legajo: "271826/2".to_owned(),
            antiguedad: 12,
            salario: 800000.00,
        }
    }

    fn build_vendendor_con_legajo(legajo: String) -> Vendedor {
        Vendedor {
            datos_personales: build_datos_personales(),
            legajo,
            antiguedad: 12,
            salario: 800000.00,
        }
    }

    fn build_cliente_con_sub() -> Cliente {
        Cliente {
            datos_personales: build_datos_personales(),
            subscripcion: Some("false@mail.com".to_owned()),
        }
    }

    fn build_cliente_sin_sub() -> Cliente {
        Cliente {
            datos_personales: build_datos_personales(),
            subscripcion: None,
        }
    }

    fn build_producto_cat01() -> Producto {
        Producto {
            nombre: "nombre_producto".to_owned(),
            categoria: "categoria01".to_owned(),
            precio_base: 5000.00,
        }
    }

    fn build_producto_cat02() -> Producto {
        Producto {
            nombre: "nombre_producto".to_owned(),
            categoria: "categoria02".to_owned(),
            precio_base: 5000.00,
        }
    }

    fn build_producto_cat03() -> Producto {
        Producto {
            nombre: "nombre_producto".to_owned(),
            categoria: "categoria03".to_owned(),
            precio_base: 5000.00,
        }
    }

    fn build_lista_productos() -> Vec<(Producto, u32)> {
        let mut list = Vec::new();
        list.push((build_producto_cat01(), 1));
        list.push((build_producto_cat01(), 2));
        list.push((build_producto_cat01(), 3));

        list
    }

    fn build_venta_con_sub() -> Venta {
        Venta {
            fecha: build_fecha(),
            productos: Vec::from([(build_producto_cat01(), 1)]),
            vendedor: build_vendendor(),
            cliente: build_cliente_con_sub(),
            medio_de_pago: MedioDePago::Efectivo,
        }
    }

    fn build_ventas_con_sub() -> Venta {
        Venta {
            fecha: build_fecha(),
            productos: Vec::from([
                (build_producto_cat01(), 1),
                (build_producto_cat02(), 2),
                (build_producto_cat03(), 3),
            ]),
            vendedor: build_vendendor(),
            cliente: build_cliente_con_sub(),
            medio_de_pago: MedioDePago::Efectivo,
        }
    }

    fn build_venta_sin_sub() -> Venta {
        Venta {
            fecha: build_fecha(),
            productos: Vec::from([(build_producto_cat01(), 1)]),
            vendedor: build_vendendor(),
            cliente: build_cliente_sin_sub(),
            medio_de_pago: MedioDePago::Efectivo,
        }
    }

    fn build_venta_por_legajo(legajo: String) -> Venta {
        Venta {
            fecha: build_fecha(),
            productos: Vec::from([(build_producto_cat01(), 1)]),
            vendedor: build_vendendor_con_legajo(legajo),
            cliente: build_cliente_sin_sub(),
            medio_de_pago: MedioDePago::Efectivo,
        }
    }

    fn build_categorias() -> HashMap<String, f64> {
        HashMap::from([
            ("categoria01".to_owned(), 0.4),
            ("categoria02".to_owned(), 0.7),
            ("categoria03".to_owned(), 1.0),
        ])
    }

    #[test]
    fn valida_agregar_una_venta() {
        let mut app = App::new();
        let venta = build_venta_con_sub();

        assert_eq!(app.ventas.len(), 0);
        app.agregar_venta(venta);
        assert_eq!(app.ventas.len(), 1);
    }

    #[test]
    fn valida_cliente_subscripto() {
        assert!(build_cliente_con_sub().esta_subscripto());
        assert!(!build_cliente_sin_sub().esta_subscripto());
    }

    #[test]
    fn valida_calcular_precio_final_de_venta() {
        let venta_cliente_con_sub = build_venta_con_sub();
        let descuento_newsletter = 30.0;

        assert!(venta_cliente_con_sub.cliente.esta_subscripto());
        assert_eq!(
            venta_cliente_con_sub.calcular_precio_final(descuento_newsletter, build_categorias()),
            3486.00,
            "Deberia hacer el siguiente calculo correctamente ((5000 - 0.4% ) * 1) - 30%"
        );

        let venta_cliente_sin_sub = build_venta_sin_sub();
        assert_eq!(
            venta_cliente_sin_sub.calcular_precio_final(descuento_newsletter, build_categorias()),
            4980.00,
            "Deberia hacer el siguiente calculo correctamente ((5000 - 0.4% ) * 1)"
        );
    }

    #[test]
    fn valida_reporte_categorias() {
        let mut app = App::new();
        app.agregar_venta(build_ventas_con_sub());

        assert_eq!(
            app.generar_reporte_categorias(),
            "categoria01: 1; categoria02: 2; categoria03: 3;".to_owned()
        );
        app.ventas.clear();
        assert_eq!(
            app.generar_reporte_categorias(),
            "No hay ventas registradas.".to_owned()
        );
    }

    #[test]
    fn valida_reporte_vendedor() {
        let mut app = App::new();
        app.agregar_venta(build_venta_por_legajo("271826/2".to_owned()));
        assert_eq!(app.generar_reporte_vendedor(), "271826/2: 1;".to_owned());

        app.agregar_venta(build_venta_por_legajo("298671/9".to_owned()));
        assert_eq!(
            app.generar_reporte_vendedor(),
            "271826/2: 1; 298671/9: 1;".to_owned()
        );

        app.ventas.clear();
        assert_eq!(
            app.generar_reporte_vendedor(),
            "No hay ventas registradas.".to_owned()
        );
    }
}
