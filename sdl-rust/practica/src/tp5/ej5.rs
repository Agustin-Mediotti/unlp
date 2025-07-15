use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use time::{Date, OffsetDateTime};

const FILE_NAME: &str = "src/tp5/archivo_subscripciones.json";

#[derive(Serialize)]
struct App {
    usuarios: HashMap<u32, Usuario>,
    estadisticas: Estadisticas,
}

impl App {
    pub fn new() -> Self {
        let mut file = File::create(FILE_NAME).unwrap();
        let app = App {
            usuarios: HashMap::new(),
            estadisticas: Estadisticas {
                medio_de_pago_mas_utilizado_subs_activas: None,
                subscripcion_mas_contratada_subs_activas: None,
                medio_de_pago_mas_utilizado: None,
                subscripcion_mas_contratada: None,
                total_medio_de_pago: HashMap::new(),
                total_subscripcion: HashMap::new(),
            },
        };
        let buf = serde_json::to_string(&app).unwrap();
        file.write_all(&buf.as_bytes())
            .expect("error escribiendo el archivo");
        app
    }

    pub fn escribir_json(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(FILE_NAME)?;
        let buf = serde_json::to_string(&self)?;
        file.write_all(&buf.as_bytes())?;
        Ok(())
    }

    pub fn create_user(&mut self, subscripcion: Subscripcion, medio_de_pago: MedioDePago) {
        if let Some(tipo) = subscripcion.tipo {
            *self
                .estadisticas
                .total_subscripcion
                .entry(tipo)
                .or_insert(0) += 1;
        }

        *self
            .estadisticas
            .total_medio_de_pago
            .entry(TipoMedioDePago::from(&medio_de_pago))
            .or_insert(0) += 1;

        let user = Usuario::new(medio_de_pago, subscripcion);
        self.usuarios.insert(self.usuarios.len() as u32, user);
        self.escribir_json().expect("error escribiendo el archivo");
    }

    pub fn cancel_subscripcion(&mut self, usuario: u32) {
        if let Some(user) = self.usuarios.iter_mut().find(|x| *x.0 == usuario) {
            user.1.cancel_subscripcion();
            self.escribir_json().expect("error escribiendo el archivo");
        }
    }

    pub fn upgrade_subscripcion(&mut self, usuario: u32) {
        if let Some(user) = self.usuarios.iter_mut().find(|x| *x.0 == usuario) {
            user.1.upgrade_sub();
            self.escribir_json().expect("error escribiendo el archivo");
        }
    }

    pub fn downgrade_subscripcion(&mut self, usuario: u32) {
        if let Some(user) = self.usuarios.iter_mut().find(|x| *x.0 == usuario) {
            user.1.downgrade_sub();
            self.escribir_json().expect("error escribiendo el archivo");
        }
    }

    pub fn update_stats(&mut self) {
        let mut med_pago_subs_act: HashMap<TipoMedioDePago, u32> = HashMap::new();
        let mut sub_mas_cont_subs_act: HashMap<TipoSubscripcion, u32> = HashMap::new();

        for usuario in &self.usuarios {
            if usuario
                .1
                .subscripcion
                .last()
                .expect("expected sub")
                .tipo
                .is_some()
            {
                if let Some(tipo) = &usuario.1.subscripcion.last().expect("expected sub").tipo {
                    *sub_mas_cont_subs_act.entry(*tipo).or_insert(0) += 1;
                }
                let tipo_pago = TipoMedioDePago::from(&usuario.1.medio_de_pago);
                *med_pago_subs_act.entry(tipo_pago).or_insert(0) += 1;
            }
        }

        self.estadisticas.subscripcion_mas_contratada = self
            .estadisticas
            .total_subscripcion
            .clone()
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        self.estadisticas.medio_de_pago_mas_utilizado = self
            .estadisticas
            .total_medio_de_pago
            .clone()
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        self.estadisticas.subscripcion_mas_contratada_subs_activas = sub_mas_cont_subs_act
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        self.estadisticas.medio_de_pago_mas_utilizado_subs_activas = med_pago_subs_act
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);
    }
}

#[derive(Serialize)]
struct Estadisticas {
    medio_de_pago_mas_utilizado_subs_activas: Option<TipoMedioDePago>,
    subscripcion_mas_contratada_subs_activas: Option<TipoSubscripcion>,
    medio_de_pago_mas_utilizado: Option<TipoMedioDePago>,
    subscripcion_mas_contratada: Option<TipoSubscripcion>,
    total_medio_de_pago: HashMap<TipoMedioDePago, u32>,
    total_subscripcion: HashMap<TipoSubscripcion, u32>,
}

#[derive(Eq, Hash, Copy, Clone, Debug, Serialize, PartialEq)]
enum TipoSubscripcion {
    Basic,
    Clasic,
    Super,
}

impl TipoSubscripcion {
    pub fn config(&self) -> (f64, u8) {
        match self {
            TipoSubscripcion::Basic => (5.0, 3),
            TipoSubscripcion::Clasic => (15.0, 12),
            TipoSubscripcion::Super => (25.0, 24),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize)]
enum TipoMedioDePago {
    Efectivo,
    MercadoPago,
    TarjetaCredito,
    Transferencia,
    Cripto,
}

impl From<&MedioDePago> for TipoMedioDePago {
    fn from(m: &MedioDePago) -> Self {
        match m {
            MedioDePago::Efectivo => TipoMedioDePago::Efectivo,
            MedioDePago::MercadoPago { .. } => TipoMedioDePago::MercadoPago,
            MedioDePago::TarjetaCredito { .. } => TipoMedioDePago::TarjetaCredito,
            MedioDePago::Transferencia { .. } => TipoMedioDePago::Transferencia,
            MedioDePago::Cripto { .. } => TipoMedioDePago::Cripto,
        }
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Serialize)]
enum MedioDePago {
    Efectivo,
    MercadoPago {
        cuentamp: String,
        cbu: String,
    },
    TarjetaCredito {
        numero_tarjeta: u64,
        numero_seguridad: u32,
        nombre_titular: String,
    },
    Transferencia {
        numero_cuenta: u64,
        nombre_titular: String,
        entidad_bancaria: String,
    },
    Cripto {
        wallet: String,
        red: String,
        tipo_token: Option<String>,
        tx_hash: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct Subscripcion {
    fecha_inicio: Date,
    costo_mensual: f64,
    duracion: u8,
    tipo: Option<TipoSubscripcion>,
    activa: bool,
}

impl Subscripcion {
    pub fn new(tipo: Option<TipoSubscripcion>) -> Self {
        let fecha_inicio = OffsetDateTime::now_utc().date();
        let activa = tipo.is_some();
        let (costo_mensual, duracion) = match &tipo {
            Some(tipo) => tipo.config(),
            None => (0.0, 0),
        };

        Subscripcion {
            fecha_inicio,
            costo_mensual,
            duracion,
            tipo,
            activa,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct Usuario {
    subscripcion: Vec<Subscripcion>,
    medio_de_pago: MedioDePago,
}

impl Usuario {
    fn new(medio_de_pago: MedioDePago, subscripcion: Subscripcion) -> Self {
        Usuario {
            subscripcion: vec![subscripcion],
            medio_de_pago,
        }
    }

    fn upgrade_sub(&mut self) {
        if let Some(sub) = self.subscripcion.last_mut() {
            match sub.tipo {
                Some(TipoSubscripcion::Basic) => {
                    sub.activa = false;
                    self.subscripcion
                        .push(Subscripcion::new(Some(TipoSubscripcion::Clasic)));
                }
                Some(TipoSubscripcion::Clasic) => {
                    sub.activa = false;
                    self.subscripcion
                        .push(Subscripcion::new(Some(TipoSubscripcion::Super)));
                }
                Some(TipoSubscripcion::Super) => (),
                None => self
                    .subscripcion
                    .push(Subscripcion::new(Some(TipoSubscripcion::Basic))),
            }
        }
    }

    fn downgrade_sub(&mut self) {
        if let Some(subscripcion) = self.subscripcion.last_mut() {
            match subscripcion.tipo {
                Some(TipoSubscripcion::Basic) => {
                    self.cancel_subscripcion();
                }
                Some(TipoSubscripcion::Clasic) => {
                    subscripcion.activa = false;
                    self.subscripcion
                        .push(Subscripcion::new(Some(TipoSubscripcion::Basic)));
                }
                Some(TipoSubscripcion::Super) => subscripcion.tipo = Some(TipoSubscripcion::Clasic),
                None => (),
            }
        }
    }

    fn cancel_subscripcion(&mut self) {
        if let Some(sub) = self.subscripcion.last_mut() {
            sub.activa = false;
            self.subscripcion.push(Subscripcion::new(None));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_fecha() -> Date {
        OffsetDateTime::now_utc().date()
    }

    fn build_subscripcion_vacia() -> Subscripcion {
        Subscripcion::new(None)
    }

    fn build_subscripcion_basic() -> Subscripcion {
        Subscripcion::new(Some(TipoSubscripcion::Basic))
    }

    fn build_subscripcion_clasic() -> Subscripcion {
        Subscripcion::new(Some(TipoSubscripcion::Clasic))
    }

    fn build_subscripcion_super() -> Subscripcion {
        Subscripcion::new(Some(TipoSubscripcion::Super))
    }

    fn build_medio_de_pago_mp() -> MedioDePago {
        MedioDePago::MercadoPago {
            cuentamp: "datosfalsos123".to_owned(),
            cbu: "12345678910111213".to_owned(),
        }
    }

    fn build_medio_de_pago_transfer() -> MedioDePago {
        MedioDePago::Transferencia {
            numero_cuenta: 12345678910111213,
            nombre_titular: "john doe".to_owned(),
            entidad_bancaria: "datosfalsos123".to_owned(),
        }
    }

    fn build_medio_de_pago_cripto() -> MedioDePago {
        MedioDePago::Cripto {
            wallet: "datosfalsos123".to_owned(),
            red: "eth".to_owned(),
            tipo_token: Some("ethereum".to_owned()),
            tx_hash: None,
        }
    }

    fn build_medio_de_pago_tarjeta() -> MedioDePago {
        MedioDePago::TarjetaCredito {
            numero_tarjeta: 123456789101112,
            numero_seguridad: 1234,
            nombre_titular: "john doe".to_owned(),
        }
    }

    fn build_usuario_unsub() -> Usuario {
        Usuario::new(MedioDePago::Efectivo, build_subscripcion_vacia())
    }

    fn build_usuario_basic() -> Usuario {
        Usuario::new(MedioDePago::Efectivo, build_subscripcion_basic())
    }

    fn build_usuario_super() -> Usuario {
        Usuario::new(MedioDePago::Efectivo, build_subscripcion_super())
    }

    #[test]
    fn verifica_crear_usuario() {
        let mut app = App::new();

        let user_con_sub = build_usuario_basic();
        let user_sin_sub = build_usuario_unsub();

        app.create_user(build_subscripcion_basic(), MedioDePago::Efectivo);

        assert_eq!(app.usuarios.len(), 1);
        assert_eq!(*app.usuarios.get(&0).unwrap(), user_con_sub);
        assert_eq!(
            app.usuarios.get(&0).unwrap().subscripcion.last(),
            Some(build_subscripcion_basic()).as_ref()
        );
        assert_eq!(
            app.usuarios.get(&0).unwrap().medio_de_pago,
            MedioDePago::Efectivo
        );

        app.create_user(build_subscripcion_vacia(), MedioDePago::Efectivo);

        assert_eq!(app.usuarios.len(), 2);
        assert_eq!(*app.usuarios.get(&1).unwrap(), user_sin_sub);
        assert_eq!(
            app.usuarios.get(&1).unwrap().subscripcion.last(),
            Some(build_subscripcion_vacia()).as_ref()
        );
        assert_eq!(
            app.usuarios.get(&1).unwrap().medio_de_pago,
            MedioDePago::Efectivo
        );
    }

    #[test]
    fn verifica_cancelar_subscripcion() {
        let mut app = App::new();

        app.create_user(build_subscripcion_basic(), MedioDePago::Efectivo);
        assert_eq!(app.usuarios.len(), 1);

        app.cancel_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            None
        );
    }

    #[test]
    fn verifica_upgrade_subscripcion() {
        let mut app = App::new();
        app.create_user(build_subscripcion_vacia(), MedioDePago::Efectivo);

        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            None
        );
        app.upgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Basic)
        );
        app.upgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Clasic)
        );
        app.upgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Super)
        );
    }

    #[test]
    fn verifica_downgrade_subscripcion() {
        let mut app = App::new();
        app.create_user(build_subscripcion_super(), MedioDePago::Efectivo);

        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Super)
        );
        app.downgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Clasic)
        );
        app.downgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Basic)
        );
        app.downgrade_subscripcion(0);
        assert_eq!(
            app.usuarios
                .get(&0)
                .unwrap()
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            None
        );
    }

    #[test]
    fn verifica_datos_estadisticos() {
        let mut app = App::new();

        app.create_user(build_subscripcion_basic(), MedioDePago::Efectivo);
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_transfer());
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_transfer());
        app.create_user(build_subscripcion_clasic(), MedioDePago::Efectivo);
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_transfer());
        app.create_user(build_subscripcion_clasic(), build_medio_de_pago_mp());
        app.create_user(build_subscripcion_clasic(), build_medio_de_pago_cripto());
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_tarjeta());
        app.usuarios.get_mut(&1).unwrap().cancel_subscripcion();
        app.usuarios.get_mut(&2).unwrap().cancel_subscripcion();
        app.usuarios.get_mut(&7).unwrap().cancel_subscripcion();

        app.update_stats();
        assert_eq!(
            app.estadisticas.medio_de_pago_mas_utilizado,
            Some(TipoMedioDePago::Transferencia)
        );
        assert_eq!(
            app.estadisticas.medio_de_pago_mas_utilizado_subs_activas,
            Some(TipoMedioDePago::Efectivo)
        );
        assert_eq!(
            app.estadisticas.subscripcion_mas_contratada,
            Some(TipoSubscripcion::Basic)
        );
        assert_eq!(
            app.estadisticas.subscripcion_mas_contratada_subs_activas,
            Some(TipoSubscripcion::Clasic)
        );
    }
}
