#[derive(Debug, PartialEq, Clone)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

pub fn con_salario_mayor<'a>(lista: Vec<Persona<'a>>, min: f64) -> Vec<Persona<'a>> {
    lista.into_iter().filter(|x| x.salario > min).collect()
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
