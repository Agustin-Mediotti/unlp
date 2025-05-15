use time::{
    Duration, {Date, Month},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Fecha {
    fecha: Date,
}

#[allow(dead_code)]
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
