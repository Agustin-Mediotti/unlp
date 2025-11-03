package edu.unlp;

import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class InversoresTest {
    protected Inversor inversor;
    protected Inversion inversion;

    private Inversion newInversionesEnAcciones(int cantAcciones, double precioPorAccion) {
        ArrayList<Accion> acciones = new ArrayList<Accion>();
        for(int i = 0; i < cantAcciones; i++) {
            acciones.add(new Accion("Mi Accion #" + i));
        }

        return new IAcciones("Paquete de #" + cantAcciones + " acciones", precioPorAccion, acciones);
    }

    @BeforeEach
    void setUp() {
        inversor = new Inversor();
    }

    @Test
    void testInversionEnAccion() {
        inversor.agregarInversion(newInversionesEnAcciones(1, 1000));

        assertEquals(1, inversor.getCantidadInversiones());
        assertEquals(1000, inversor.getMontoInversionTotal());

        inversor.agregarInversion(newInversionesEnAcciones(2, 800));

        assertEquals(2, inversor.getCantidadInversiones());
        assertEquals(2600, inversor.getMontoInversionTotal());
    }

    @Test
    void testInversionEnPlazoFijo() {
        inversor.agregarInversion(new IPlazoFijo(50_000, 3));
        
        assertEquals(1, inversor.getCantidadInversiones());
        assertEquals(50_000, inversor.getMontoInversionTotal());

        inversor.agregarInversion(new IPlazoFijo(100_000, 10));
        assertEquals(2, inversor.getCantidadInversiones());
        assertEquals(150_000, inversor.getMontoInversionTotal());
    }

    @Test
    void testInversionesMixtas() {
        inversor.agregarInversion(new IPlazoFijo(50_000, 3));
        inversor.agregarInversion(newInversionesEnAcciones(5, 2000));

        assertEquals(60_000, inversor.getMontoInversionTotal());
        assertEquals(2, inversor.getCantidadInversiones());
    }
}
