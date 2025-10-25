package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class CajaDeAhorroTest {

    Cuenta cuenta;
    Cuenta otraCuenta;
    CuentaCorriente cuentaCorriente;

    @BeforeEach
    void setUp() throws Exception {
        cuenta = new CajaDeAhorro();
        otraCuenta = new CajaDeAhorro();
    }

    @Test
    public void testDepositarCajaDeAhorro() {
        assertEquals(0, cuenta.getSaldo());

        cuenta.depositar(10000); // Saldo: 10000 - 200 = 9800
        assertEquals(9800, cuenta.getSaldo());
    }

    @Test
    public void testExtraccionCajaDeAhorro() {
        cuenta.depositar(10000);
        assertEquals(9800, cuenta.getSaldo());

        assert (cuenta.puedeExtraerMonto(5100));
        cuenta.extraer(5000); // Saldo: 9800 - 5100 (monto: 5000 + 2%) = 4700
        assertEquals(4700, cuenta.getSaldo());

        assert (!cuenta.puedeExtraerMonto(100_000_000));
    }

    @Test
    public void testTransferirCajaDeAhorro() {
        otraCuenta.depositar(10000);
        assertEquals(9800, otraCuenta.getSaldo());

        assert (otraCuenta.transferirACuenta(5000, cuenta));
        assertEquals(4900, cuenta.getSaldo());
        assertEquals(4700, otraCuenta.getSaldo());
    }

    @Test
    public void testTransferirCajaDeAhorroACuentaCorriente() {
        cuentaCorriente = new CuentaCorriente();
        
        cuenta.depositar(10000);
        assertEquals(9800, cuenta.getSaldo());
        
        assert(cuenta.transferirACuenta(5000, cuentaCorriente));
        assertEquals(4700, cuenta.getSaldo());
        assertEquals(5000, cuentaCorriente.getSaldo());
    }
}