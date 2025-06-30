use rand::Rng;
use std::collections::HashMap;
use time::OffsetDateTime;

const ERROR_COTIZACION: &str = "Error obteniendo cotizacion";
const ERROR_CRIPTOMONEDA: &str = "Error obteniendo criptomoneda";
const ERROR_BLOCKCHAIN: &str = "Error obteniendo blockchain";

struct Blockchain {
    nombre: String,
    prefijo: String,
}

impl Blockchain {
    fn generar_hash(&self) -> String {
        format!("{}{}", self.nombre, rand::rng().random::<u32>())
    }
}

struct Criptomoneda {
    nombre: String,
    prefijo: String,
    blockchains: HashMap<String, Blockchain>, // clave: prefijo_blockchain
}

enum MedioDePago {
    MercadoPago,
    Transferencia,
}

struct Balance {
    user_id: String,
    balance_fiat: f64,
    balance_cripto: HashMap<String, f64>, // clave: prefijo_cripto
}

impl Balance {
    fn vender_cripto(&mut self, pref_cripto: &str, monto_fiat: f64, cotizacion: f64) {
        self.balance_fiat += monto_fiat;
        self.balance_cripto
            .entry(pref_cripto.to_owned())
            .and_modify(|c| *c += monto_fiat / cotizacion)
            .or_insert(monto_fiat / cotizacion);
    }

    fn comprar_cripto(&mut self, pref_cripto: &str, monto_fiat: f64, cotizacion: f64) {
        self.balance_fiat -= monto_fiat;
        self.balance_cripto
            .entry(pref_cripto.to_owned())
            .and_modify(|c| *c += monto_fiat / cotizacion)
            .or_insert(monto_fiat / cotizacion);
    }

    fn retirar_fiat(&mut self, monto_fiat: f64) {
        self.balance_fiat -= monto_fiat;
    }
}

struct Usuario {
    user_id: String,
    nombre: String,
    apellido: String,
    dni: String,
    email: String,
    validado: bool,
}

enum TipoTransaccion {
    IngresoDinero {
        monto: f64,
    },
    CompraCripto {
        cripto: String,
        monto: f64,
        cotizacion: f64,
    },
    VentaCripto {
        cripto: String,
        monto: f64,
        cotizacion: f64,
    },
    RetirarFiat {
        medio_pago: MedioDePago,
    },
    RetiroCripto {
        cripto: String,
        monto: f64,
        hash: String,
        cotizacion: f64,
    },
    RecepcionCripto {
        cripto: String,
        monto: f64,
        cotizacion: f64,
    },
}

struct Transaccion {
    user_id: String,
    monto: f64,
    tipo: TipoTransaccion,
}

impl Transaccion {
    fn new(tipo: TipoTransaccion, monto: f64, user_id: String) -> Self {
        Transaccion {
            tipo,
            monto,
            user_id,
        }
    }
}

struct Cotizacion {
    fecha: OffsetDateTime,
    compra: f64,
    venta: f64,
}

impl Cotizacion {
    pub fn new(compra: f64, venta: f64) -> Self {
        Cotizacion {
            fecha: OffsetDateTime::now_utc(),
            compra,
            venta,
        }
    }
}

struct Estadisiticas {
    cant_ventas: u32,
    cant_compras: u32,
    volumen_ventas: f64,
    volumen_compras: f64,
}

struct App {
    usuarios: HashMap<String, Usuario>,             // clave: user_id
    balances: HashMap<String, Balance>,             // clave: user_id
    criptomonedas: HashMap<String, Criptomoneda>,   // clave: prefijo_cripto
    cotizaciones: HashMap<String, Vec<Cotizacion>>, // clave: prefijo_cripto
    estadisticas: HashMap<String, Estadisiticas>,   // clave: prefijo_cripto
    transacciones: Vec<Transaccion>,
}

impl App {
    pub fn new() -> Self {
        App {
            usuarios: HashMap::new(),
            balances: HashMap::new(),
            criptomonedas: HashMap::new(),
            cotizaciones: HashMap::new(),
            transacciones: Vec::new(),
            estadisticas: HashMap::new(),
        }
    }
    pub fn obtener_cotizacion(&self, pref_cripto: &str) -> &Cotizacion {
        self.cotizaciones
            .get(pref_cripto)
            .and_then(|f| Some(f.last().expect(ERROR_COTIZACION)))
            .expect("err")
    }

    pub fn ingresar_dinero(&mut self, monto_fiat: f64, user_id: String) {
        if self.balances.contains_key(&user_id) {
            self.balances
                .entry(user_id.clone())
                .and_modify(|u| u.balance_fiat += monto_fiat);

            self.transacciones.push(Transaccion::new(
                TipoTransaccion::IngresoDinero { monto: monto_fiat },
                monto_fiat,
                user_id,
            ));
        }
    }

    pub fn comprar_cripto(&mut self, pref_cripto: String, monto_fiat: f64, user_id: String) {
        // obtengo cotizacion actual
        let cotizacion = self.obtener_cotizacion(&pref_cripto).compra;
        // verficar que usuario existe
        if let Some(balance) = self.balances.get_mut(&user_id) {
            // verifica que el usuario esta validado
            if self.usuarios.get(&user_id).is_some_and(|u| u.validado) {
                // verificar qeu tiene saldo para transaccion
                if balance.balance_fiat >= monto_fiat {
                    // hago la compra
                    balance.comprar_cripto(&pref_cripto, monto_fiat, cotizacion);
                    // y genero transaccion
                    self.transacciones.push(Transaccion::new(
                        TipoTransaccion::CompraCripto {
                            cripto: pref_cripto,
                            monto: monto_fiat / cotizacion,
                            cotizacion,
                        },
                        monto_fiat,
                        user_id,
                    ))
                }
            }
        }
    }

    pub fn vender_cripto(&mut self, pref_cripto: String, monto_fiat: f64, user_id: String) {
        // obtengo cotizacion actual
        let cotizacion = self.obtener_cotizacion(&pref_cripto).venta;
        // verificar que el usuario existe
        if let Some(balance) = self.balances.get_mut(&user_id) {
            // verifica que el usuario esta validado
            if self.usuarios.get(&user_id).is_some_and(|u| u.validado) {
                // verifico que el usuario tiene saldo suficiente para la transaccion
                if let Some(balance_actual) = balance.balance_cripto.get(&pref_cripto) {
                    if *balance_actual >= monto_fiat / cotizacion {
                        // hago la venta
                        balance.vender_cripto(&pref_cripto, monto_fiat, cotizacion);
                        // y generop la transaccion
                        self.transacciones.push(Transaccion::new(
                            TipoTransaccion::VentaCripto {
                                cripto: pref_cripto,
                                monto: monto_fiat / cotizacion,
                                cotizacion,
                            },
                            monto_fiat,
                            user_id,
                        ))
                    }
                }
            }
        }
    }

    pub fn retirar_fiat(&mut self, user_id: String, monto_fiat: f64, medio: MedioDePago) {
        if self.usuarios.get(&user_id).is_some_and(|u| u.validado) {
            if let Some(balance) = self.balances.get_mut(&user_id) {
                if balance.balance_fiat >= monto_fiat {
                    balance.retirar_fiat(monto_fiat);
                    self.transacciones.push(Transaccion::new(
                        TipoTransaccion::RetirarFiat { medio_pago: medio },
                        monto_fiat,
                        user_id,
                    ));
                }
            }
        }
    }

    pub fn recibir_hash(&self, pref_cripto: &str, blockchain: &str) -> String {
        self.criptomonedas
            .get(pref_cripto)
            .expect(ERROR_CRIPTOMONEDA)
            .blockchains
            .get(blockchain)
            .expect(ERROR_BLOCKCHAIN)
            .generar_hash()
    }

    pub fn retirar_cripto(
        &mut self,
        user_id: String,
        pref_cripto: String,
        blockchain: String,
        monto_cripto: f64,
    ) {
        if self.usuarios.get(&user_id).is_some_and(|u| u.validado) {
            if let Some(balance) = self.balances.get_mut(&user_id) {
                if let Some(balance_actual) = balance.balance_cripto.get_mut(&pref_cripto) {
                    if *balance_actual >= monto_cripto {
                        *balance_actual -= monto_cripto;
                        let cotizacion = self.obtener_cotizacion(&pref_cripto).venta;
                        self.transacciones.push(Transaccion {
                            user_id,
                            monto: monto_cripto / cotizacion,
                            tipo: TipoTransaccion::RetiroCripto {
                                hash: self.recibir_hash(&pref_cripto, &blockchain),
                                cripto: pref_cripto,
                                monto: monto_cripto,
                                cotizacion,
                            },
                        })
                    }
                }
            }
        }
    }

    pub fn recibir_cripto(
        &mut self,
        user_id: String,
        pref_cripto: String,
        _blockchain: String,
        monto_cripto: f64,
    ) {
        if self.usuarios.get(&user_id).is_some_and(|u| u.validado) {
            if let Some(balance) = self.balances.get_mut(&user_id) {
                if let Some(balance_actual) = balance.balance_cripto.get_mut(&pref_cripto) {
                    *balance_actual += monto_cripto;
                    let cotizacion = self.obtener_cotizacion(&pref_cripto).venta;

                    self.transacciones.push(Transaccion {
                        user_id,
                        monto: monto_cripto / cotizacion,
                        tipo: TipoTransaccion::RecepcionCripto {
                            cripto: pref_cripto,
                            monto: monto_cripto,
                            cotizacion,
                        },
                    })
                }
            }
        }
    }

    pub fn actualizar_datos_estadisticos(&mut self) {
        self.transacciones.iter().for_each(|t| match &t.tipo {
            TipoTransaccion::CompraCripto { cripto, monto, .. } => {
                self.estadisticas.entry(cripto.to_owned()).and_modify(|e| {
                    e.cant_compras += 1;
                    e.volumen_compras += monto;
                });
            }
            TipoTransaccion::VentaCripto { cripto, monto, .. } => {
                self.estadisticas.entry(cripto.to_owned()).and_modify(|e| {
                    e.cant_ventas += 1;
                    e.volumen_ventas += monto;
                });
            }
            _ => (),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_usuario_validado(user_id: &str) -> Usuario {
        Usuario {
            user_id: user_id.to_string(),
            nombre: "Agustin".to_string(),
            apellido: "Mediotti".to_string(),
            dni: "12345678".to_string(),
            email: "agus@test.com".to_string(),
            validado: true,
        }
    }

    fn build_balance_con_fiat(user_id: &str, fiat: f64) -> Balance {
        Balance {
            user_id: user_id.to_string(),
            balance_fiat: fiat,
            balance_cripto: HashMap::new(),
        }
    }

    fn build_cotizaciones(compra: f64, venta: f64) -> Vec<Cotizacion> {
        vec![Cotizacion::new(compra, venta)]
    }

    fn build_cripto_con_blockchain() -> Criptomoneda {
        let mut blockchains = HashMap::new();
        blockchains.insert(
            "btc".to_string(),
            Blockchain {
                nombre: "BTCNET".to_string(),
                prefijo: "btc".to_string(),
            },
        );

        Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "btc".to_string(),
            blockchains,
        }
    }

    fn setup_app(user_id: &str, compra: f64, venta: f64, fiat: f64) -> App {
        let mut app = App::new();

        app.usuarios
            .insert(user_id.to_string(), build_usuario_validado(user_id));
        app.balances
            .insert(user_id.to_string(), build_balance_con_fiat(user_id, fiat));
        app.cotizaciones
            .insert("btc".to_string(), build_cotizaciones(compra, venta));
        app.criptomonedas
            .insert("btc".to_string(), build_cripto_con_blockchain());

        app.estadisticas.insert(
            "btc".to_string(),
            Estadisiticas {
                cant_compras: 0,
                cant_ventas: 0,
                volumen_compras: 0.0,
                volumen_ventas: 0.0,
            },
        );

        app
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 1000.0);
        app.ingresar_dinero(2000.0, "user1".to_string());

        assert_eq!(app.balances["user1"].balance_fiat, 3000.0);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 1000.0);
        app.comprar_cripto("btc".to_string(), 500.0, "user1".to_string());

        assert!(app.balances["user1"].balance_cripto.contains_key("btc"));
        assert!(app.balances["user1"].balance_fiat < 1000.0);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_vender_cripto() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 0.0);

        app.balances
            .get_mut("user1")
            .unwrap()
            .balance_cripto
            .insert("btc".to_string(), 0.01);
        app.vender_cripto("btc".to_string(), 520.0, "user1".to_string());

        assert!(app.balances["user1"].balance_fiat > 0.0);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_fiat() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 1000.0);
        app.retirar_fiat("user1".to_string(), 500.0, MedioDePago::Transferencia);

        assert_eq!(app.balances["user1"].balance_fiat, 500.0);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 0.0);
        app.balances
            .get_mut("user1")
            .unwrap()
            .balance_cripto
            .insert("btc".to_string(), 0.01);
        app.retirar_cripto(
            "user1".to_string(),
            "btc".to_string(),
            "btc".to_string(),
            0.005,
        );

        assert!(app.balances["user1"].balance_cripto["btc"] < 0.01);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 0.0);
        app.balances
            .get_mut("user1")
            .unwrap()
            .balance_cripto
            .insert("btc".to_string(), 0.0);
        app.recibir_cripto(
            "user1".to_string(),
            "btc".to_string(),
            "btc".to_string(),
            0.003,
        );

        assert_eq!(app.balances["user1"].balance_cripto["btc"], 0.003);
        assert_eq!(app.transacciones.len(), 1);
    }

    #[test]
    fn test_actualizar_datos_estadisticos() {
        let mut app = setup_app("user1", 50000.0, 52000.0, 1000.0);
        app.comprar_cripto("btc".to_string(), 500.0, "user1".to_string());
        app.vender_cripto("btc".to_string(), 200.0, "user1".to_string());

        app.actualizar_datos_estadisticos();

        let stats = &app.estadisticas["btc"];
        assert_eq!(stats.cant_compras, 1);
        assert_eq!(stats.cant_ventas, 1);
        assert!(stats.volumen_compras > 0.0);
        assert!(stats.volumen_ventas > 0.0);
    }
}
