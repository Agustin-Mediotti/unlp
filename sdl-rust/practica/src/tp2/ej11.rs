#[allow(dead_code)]
pub fn multiplicar_valores(lista: &mut [i32], factor: i32) {
    lista.iter_mut().for_each(|x| *x *= factor);
}
