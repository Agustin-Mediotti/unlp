use std::collections::HashMap;
use time::{Date, OffsetDateTime};

struct App {
    usuarios: Vec<Usuario>,
    estadisticas: Estadisticas,
}

impl App {
    pub fn new() -> Self {
        App {
            usuarios: Vec::new(),
            estadisticas: Estadisticas {
                medio_de_pago_mas_utilizado_subs_activas: None,
                subscripcion_mas_contratada_subs_activas: None,
                medio_de_pago_mas_utilizado: None,
                subscripcion_mas_contratada: None,
                total_medio_de_pago: HashMap::new(),
                total_subscripcion: HashMap::new(),
            },
        }
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
        self.usuarios.push(user);
    }

    pub fn cancel_subscripcion(&mut self, usuario: &Usuario) {
        if let Some(user) = self.usuarios.iter_mut().find(|x| **x == *usuario) {
            user.cancel_subscripcion();
        }
    }

    pub fn update_stats(&mut self) {
        let mut med_pago_subs_act: HashMap<TipoMedioDePago, u32> = HashMap::new();
        let mut sub_mas_cont_subs_act: HashMap<TipoSubscripcion, u32> = HashMap::new();

        for usuario in &self.usuarios {
            if usuario
                .subscripcion
                .last()
                .expect("expected sub")
                .tipo
                .is_some()
            {
                if let Some(tipo) = &usuario.subscripcion.last().expect("expected sub").tipo {
                    *sub_mas_cont_subs_act.entry(*tipo).or_insert(0) += 1;
                }
                let tipo_pago = TipoMedioDePago::from(&usuario.medio_de_pago);
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

struct Estadisticas {
    medio_de_pago_mas_utilizado_subs_activas: Option<TipoMedioDePago>,
    subscripcion_mas_contratada_subs_activas: Option<TipoSubscripcion>,
    medio_de_pago_mas_utilizado: Option<TipoMedioDePago>,
    subscripcion_mas_contratada: Option<TipoSubscripcion>,
    total_medio_de_pago: HashMap<TipoMedioDePago, u32>,
    total_subscripcion: HashMap<TipoSubscripcion, u32>,
}

#[derive(Eq, Hash, Copy, Clone, Debug)]
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

impl PartialEq for TipoSubscripcion {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
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

#[derive(Hash, Debug)]
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

impl PartialEq for MedioDePago {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::MercadoPago {
                    cuentamp: l_cuentamp,
                    cbu: l_cbu,
                },
                Self::MercadoPago {
                    cuentamp: r_cuentamp,
                    cbu: r_cbu,
                },
            ) => l_cuentamp == r_cuentamp && l_cbu == r_cbu,
            (
                Self::TarjetaCredito {
                    numero_tarjeta: l_numero_tarjeta,
                    numero_seguridad: l_numero_seguridad,
                    nombre_titular: l_nombre_titular,
                },
                Self::TarjetaCredito {
                    numero_tarjeta: r_numero_tarjeta,
                    numero_seguridad: r_numero_seguridad,
                    nombre_titular: r_nombre_titular,
                },
            ) => {
                l_numero_tarjeta == r_numero_tarjeta
                    && l_numero_seguridad == r_numero_seguridad
                    && l_nombre_titular == r_nombre_titular
            }
            (
                Self::Transferencia {
                    numero_cuenta: l_numero_cuenta,
                    nombre_titular: l_nombre_titular,
                    entidad_bancaria: l_entidad_bancaria,
                },
                Self::Transferencia {
                    numero_cuenta: r_numero_cuenta,
                    nombre_titular: r_nombre_titular,
                    entidad_bancaria: r_entidad_bancaria,
                },
            ) => {
                l_numero_cuenta == r_numero_cuenta
                    && l_nombre_titular == r_nombre_titular
                    && l_entidad_bancaria == r_entidad_bancaria
            }
            (
                Self::Cripto {
                    wallet: l_wallet,
                    red: l_red,
                    tipo_token: l_tipo_token,
                    tx_hash: l_tx_hash,
                },
                Self::Cripto {
                    wallet: r_wallet,
                    red: r_red,
                    tipo_token: r_tipo_token,
                    tx_hash: r_tx_hash,
                },
            ) => {
                l_wallet == r_wallet
                    && l_red == r_red
                    && l_tipo_token == r_tipo_token
                    && l_tx_hash == r_tx_hash
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Clone)]
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

impl PartialEq for Subscripcion {
    fn eq(&self, other: &Self) -> bool {
        self.fecha_inicio == other.fecha_inicio
            && self.costo_mensual == other.costo_mensual
            && self.duracion == other.duracion
            && self.tipo == other.tipo
    }
}

#[derive(Debug)]
struct Usuario {
    subscripcion: Vec<Subscripcion>,
    medio_de_pago: MedioDePago,
}

impl PartialEq for Usuario {
    fn eq(&self, other: &Self) -> bool {
        self.subscripcion == other.subscripcion && self.medio_de_pago == other.medio_de_pago
    }
}

impl Usuario {
    fn new(medio_de_pago: MedioDePago, subscripcion: Subscripcion) -> Self {
        Usuario {
            subscripcion: vec![subscripcion],
            medio_de_pago,
        }
    }

    fn upgrade_sub(&mut self) {
        if let Some(subscripcion) = self.subscripcion.last_mut() {
            match subscripcion.tipo {
                Some(TipoSubscripcion::Basic) => {
                    subscripcion.activa = false;
                    self.subscripcion
                        .push(Subscripcion::new(Some(TipoSubscripcion::Clasic)));
                }
                Some(TipoSubscripcion::Clasic) => {
                    subscripcion.activa = false;
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

    fn build_usuario_basic() -> Usuario {
        Usuario::new(MedioDePago::Efectivo, build_subscripcion_basic())
    }

    fn build_usuario_super() -> Usuario {
        Usuario::new(MedioDePago::Efectivo, build_subscripcion_super())
    }

    #[test]
    fn verifica_crear_usuario() {
        let mut app = App::new();

        let sub = build_subscripcion_basic();
        let expect = build_usuario_basic();

        app.create_user(build_subscripcion_basic(), MedioDePago::Efectivo);

        assert_eq!(app.usuarios.len(), 1);
        assert_eq!(app.usuarios[0], expect);
        assert_eq!(app.usuarios[0].subscripcion.last(), Some(sub).as_ref());
        assert_eq!(app.usuarios[0].medio_de_pago, MedioDePago::Efectivo);
    }

    #[test]
    fn verifica_cancelar_subscripcion() {
        let mut app = App::new();
        let usuario = build_usuario_basic();

        app.create_user(build_subscripcion_basic(), MedioDePago::Efectivo);
        assert_eq!(app.usuarios.len(), 1);

        app.cancel_subscripcion(&usuario);
        assert_eq!(
            app.usuarios[0]
                .subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            None
        );
    }

    #[test]
    fn verifica_upgrade_subscripcion() {
        let mut user = build_usuario_basic();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Basic)
        );
        user.upgrade_sub();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Clasic)
        );
        user.upgrade_sub();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Super)
        );
    }

    #[test]
    fn verifica_downgrade_subscripcion() {
        let mut user = build_usuario_super();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Super)
        );
        user.downgrade_sub();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Clasic)
        );
        user.downgrade_sub();
        assert_eq!(
            user.subscripcion
                .last()
                .expect("expected subscription")
                .tipo,
            Some(TipoSubscripcion::Basic)
        );
        user.downgrade_sub();
        assert_eq!(
            user.subscripcion
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
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_tarjeta());
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_tarjeta());
        app.create_user(build_subscripcion_clasic(), MedioDePago::Efectivo);
        app.create_user(build_subscripcion_basic(), build_medio_de_pago_tarjeta());
        app.create_user(build_subscripcion_clasic(), build_medio_de_pago_mp());
        app.create_user(build_subscripcion_clasic(), build_medio_de_pago_cripto());
        app.usuarios[1].cancel_subscripcion();
        app.usuarios[2].cancel_subscripcion();

        app.update_stats();
        assert_eq!(
            app.estadisticas.medio_de_pago_mas_utilizado,
            Some(TipoMedioDePago::TarjetaCredito)
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
