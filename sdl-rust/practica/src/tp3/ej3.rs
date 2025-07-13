use serde::Serialize;
use time::{
    Duration, {Date, Month},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize)]
pub struct Fecha {
    fecha: Date,
}

impl Fecha {
    pub fn new(dia: u8, mes: u8, anio: i32) -> Option<Self> {
        if Self::es_fecha_valida(dia, mes, anio) {
            Some(Self {
                fecha: Date::from_calendar_date(anio, Month::try_from(mes).unwrap(), dia).unwrap(),
            })
        } else {
            None
        }
    }

    pub fn es_fecha_valida(dia: u8, mes: u8, anio: i32) -> bool {
        if let Ok(month) = Month::try_from(mes) {
            Date::from_calendar_date(anio, month, dia).is_ok()
        } else {
            false
        }
    }

    pub fn es_bisiesto(&self) -> bool {
        /*
            https://es.wikipedia.org/wiki/A%C3%B1o_bisiesto

           p: Es divisible por 4
           q: Es divisible por 100
           r: Es divisible por 400

           (p ∧ ~q) ∨ r
        */
        (self.fecha.year() % 4 == 0 && self.fecha.year() & 100 != 0) || self.fecha.year() & 400 == 0
    }

    pub fn sumar_dias(&mut self, dias: i64) {
        self.fecha = self.fecha + Duration::days(dias);
    }

    pub fn restar_dias(&mut self, dias: i64) {
        self.fecha = self.fecha - Duration::days(dias);
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        self.fecha > una_fecha.fecha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crea_una_fecha_con_correctamente() {
        assert!(Fecha::new(11, 09, 2001).is_some());
        assert!(Fecha::new(31, 11, 2025).is_none());
        assert!(Fecha::new(29, 2, 2023).is_none());
    }

    #[test]
    fn verifica_fecha_valida() {
        assert!(!Fecha::es_fecha_valida(29, 2, 2023));
        assert!(!Fecha::es_fecha_valida(31, 11, 2025));
        assert!(Fecha::es_fecha_valida(31, 12, 2025));
    }

    #[test]
    fn verifica_fecha_bisiesto() {
        assert!(
            Fecha::new(29, 3, 2024)
                .expect("Fecha incorrecta")
                .es_bisiesto()
        );
        assert!(
            !Fecha::new(19, 5, 2023)
                .expect("Fecha incorrecta")
                .es_bisiesto()
        );
    }

    #[test]
    fn verifica_sumar_dias_a_fecha() {
        let mut fecha = Fecha::new(11, 09, 2001).unwrap();
        fecha.sumar_dias(15);

        assert_eq!(fecha, Fecha::new(26, 09, 2001).unwrap());
    }

    #[test]
    fn verifica_restar_dias_a_fecha() {
        let mut fecha = Fecha::new(11, 09, 2001).unwrap();
        fecha.restar_dias(2);

        assert_eq!(fecha, Fecha::new(09, 09, 2001).unwrap());
    }
}
