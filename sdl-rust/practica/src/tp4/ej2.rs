#[derive(Debug, Clone)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl<'a> PartialEq for Persona<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre
            && self.apellido == other.apellido
            && self.ciudad == other.ciudad
            && self.edad == other.edad
            && self.direccion == other.direccion
            && self.salario == other.salario
    }
}

pub fn con_salario_mayor<'a>(lista: Vec<Persona<'a>>, min: f64) -> Vec<Persona<'a>> {
    lista.into_iter().filter(|x| x.salario > min).collect()
}

pub fn edad_mayor_ciudad_igual<'a>(
    lista: Vec<Persona<'a>>,
    edad: u8,
    ciudad: &str,
) -> Vec<Persona<'a>> {
    lista
        .into_iter()
        .filter(|e| (e.edad > edad) && (e.ciudad == ciudad))
        .collect()
}

pub fn pertenecen_a_ciudad<'a>(lista: Vec<Persona<'a>>, ciudad: &str) -> bool {
    lista.into_iter().all(|f| f.ciudad == ciudad)
}

pub fn almenos_uno_pertenece<'a>(lista: Vec<Persona<'a>>, ciudad: &str) -> bool {
    lista.into_iter().any(|f| f.ciudad == ciudad)
}

pub fn existe_en_arreglo<'a>(lista: Vec<Persona<'a>>, persona: Persona<'a>) -> bool {
    lista.into_iter().find(|e| *e == persona).is_some()
}

pub fn edades<'a>(lista: Vec<Persona<'a>>) -> Vec<u8> {
    let mut edades: Vec<u8> = Vec::new();
    lista.into_iter().for_each(|f| edades.push(f.edad));
    edades
}

pub fn menor_mayor_salario<'a>(lista: Vec<Persona<'a>>) -> Option<(Persona<'a>, Persona<'a>)> {
    if lista.is_empty() {
        return None;
    }

    let menor = lista
        .iter()
        .min_by(|a, b| {
            a.salario
                .partial_cmp(&b.salario)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.edad.cmp(&a.edad))
        })
        .map(|p| p.clone())?;

    let mayor = lista
        .iter()
        .max_by(|a, b| {
            a.salario
                .partial_cmp(&b.salario)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.edad.cmp(&b.edad)) // mayor edad preferida
        })
        .map(|p| p.clone())?;

    Some((menor, mayor))
}

#[test]
fn verifica_salario_mayor() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01, persona02.clone()]);
    let lista_vacia = Vec::new();
    assert_eq!(
        con_salario_mayor(lista.clone(), 750_000.00),
        [persona02.clone()],
        "Debería devolver la persona con salario superior al parámetro"
    );
    assert_eq!(
        con_salario_mayor(lista_vacia, 500_000.00),
        [],
        "Debería devolver una lista vacía"
    );
    assert_eq!(
        con_salario_mayor(lista, 560_000.00),
        [persona02],
        "Debería devolver solo la pesona con salario superior al parámetro"
    );
}

#[test]
fn verifica_edad_mayor_ciudad_igual() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02]);
    let lista_vacia = Vec::new();
    assert_eq!(
        edad_mayor_ciudad_igual(lista, 21, "twin peaks"),
        [persona01],
        "Debería devolver la persona con edad superior al parámetro y ciudad igual al parámetro"
    );
    assert_eq!(
        edad_mayor_ciudad_igual(lista_vacia, 35, "Calle falsa 123"),
        [],
        "Debería devolver una lista vacía"
    );
}

#[test]
fn verifica_pertenecen_a_ciudad() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "twin peaks",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02]);
    let lista_vacia = Vec::new();
    assert!(
        pertenecen_a_ciudad(lista, "twin peaks"),
        "Debería devolver verdadero una lista con personas de la misma ciudad"
    );
    assert!(
        pertenecen_a_ciudad(lista_vacia, "twin peaks"),
        "Debería devolver falso a una lista vacía"
    );
}

#[test]
fn verifica_almenos_uno_pertenece() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02]);
    let lista_vacia = Vec::new();
    assert!(
        almenos_uno_pertenece(lista, "twin peaks"),
        "Debería devolver verdadero una lista con una persona de la ciudad pasada por parámetro"
    );
    assert!(
        !almenos_uno_pertenece(lista_vacia, "twin peaks"),
        "Debería devolver falso a una lista vacía"
    );
}

#[test]
fn verifica_existe_en_arreglo() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02.clone()]);
    let lista_vacia = Vec::new();
    assert!(
        existe_en_arreglo(lista, persona02),
        "Debería devolver verdadero una lista que contiene la persona pasada por parámetro"
    );
    assert!(
        !existe_en_arreglo(lista_vacia, persona01),
        "Debería devolver falso a una lista vacía"
    );
}

#[test]
fn verifica_edades() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02.clone()]);
    let lista_vacia = Vec::new();
    assert_eq!(
        edades(lista),
        [25, 35],
        "Debería devolver una lista con las edades de las personas en la lista"
    );
    assert_eq!(
        edades(lista_vacia),
        Vec::<u8>::new(),
        "Debería devolver una lista vacía"
    );
}

#[test]
fn verifica_mayor_menor_salario() {
    let persona01 = Persona {
        nombre: "laura",
        apellido: "palmer",
        direccion: "44st & Washington",
        ciudad: "twin peaks",
        salario: 560_000.00,
        edad: 25,
    };
    let persona02 = Persona {
        nombre: "karen",
        apellido: "pensiones",
        direccion: "calle falsa 123",
        ciudad: "santiago del estero",
        salario: 1_250_000.00,
        edad: 35,
    };
    let lista = Vec::from([persona01.clone(), persona02.clone()]);
    let lista_vacia = Vec::new();
    assert_eq!(
        menor_mayor_salario(lista),
        Some((persona01.clone(), persona02.clone())),
        "Debería devolver Some de una tupla con las personas de menor y mayor salario"
    );
    assert_eq!(
        menor_mayor_salario(lista_vacia),
        None,
        "Debería devolver None"
    );
}
