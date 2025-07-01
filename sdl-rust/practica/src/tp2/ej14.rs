pub fn incrementar(num: &mut f32) {
    *num += 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incrementa_variable_de_punto_flotante() {
        let mut num: f32 = 2.0;
        incrementar(&mut num);
        assert_eq!(
            num, 3.0,
            "La variable {num} debería haber sido incrementada a 3.0"
        );
    }
}
