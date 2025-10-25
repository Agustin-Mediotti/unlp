package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class CuentaCorrienteTest {

    CuentaCorriente cuenta;
    Cuenta otraCuenta;

    @BeforeEach
    void setUp() throws Exception {
        cuenta = new CuentaCorriente();
        otraCuenta = new CajaDeAhorro();
    }

    @Test
    public void testExtraccionCuentaCorriente() {
        cuenta.setLimite(5000);
        assertEquals(5000, cuenta.getLimite());
        assertEquals(0, cuenta.getDescubierto());
        
        assert(cuenta.extraer(5000)); // Extraccion del total
        assertEquals(5000, cuenta.getDescubierto());
        assert(!cuenta.extraer(5000)); // No se debe poder extraer más del límite
    }

    @Test
    public void testTransferirCuentaCorriente() {
        cuenta.depositar(5000);
        assertEquals(5000, cuenta.getSaldo());

        assert(cuenta.transferirACuenta(5000, otraCuenta)); // Transferencia del total
        assertEquals(0, cuenta.getSaldo());
        assertEquals(4900, otraCuenta.getSaldo()); // total - 2% por valor del deposito
    }
}
