#[allow(dead_code)]
pub fn reemplazar_pares(lista: &mut [i32]) {
    lista
        .iter_mut()
        .filter(|e| **e % 2 == 0)
        .for_each(|e| *e = -1);
}
