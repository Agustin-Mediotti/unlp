pub trait I32Primos {
    fn es_primo(&self) -> bool;
}
impl I32Primos for i32 {
    fn es_primo(&self) -> bool {
        if *self < 2 {
            return false;
        }
        if *self == 2 {
            return true;
        }
        if *self % 2 == 0 {
            return false;
        }
        let limite = (*self as f64).sqrt() as i32 + 1;
        for i in (3..limite).step_by(2) {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}

pub fn cant_primos(vec: &[i32]) -> i32 {
    vec.iter().filter(|&&e| e.es_primo()).count() as i32
}

#[test]
fn verifica_cantidad_primos_correctamente() {
    assert_eq!(cant_primos(&[2, 3, 4, 5, 6, 7]), 4);
}
