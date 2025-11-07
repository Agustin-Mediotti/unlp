package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class UsuarioTest {

    private Usuario user;

    @BeforeEach
    void setUp() throws Exception {
        user = new Usuario("Nullencio Exceptionale", 19201928, 1000);
    }

    @Test
    public void testDescontarCreditoSaldoSuficiente() {
        assertEquals(0, user.descontarCredito(500));
    }

    @Test
    public void testDescontarCreditoSaldoInsuficiente() {
        assertEquals(200, user.descontarCredito(1200));
    }
}
