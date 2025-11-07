package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class Impresion3DTest {
    
    private Impresion3D file3d;

    @BeforeEach
    void setUp() throws Exception {
        file3d = new Impresion3D("file3d.gcode", 500);
    }

    @Test
    public void testCalcularDuracion() {
        assertEquals(5, file3d.calcularDuracion());
    }

    @Test
    public void testCalcularCosto() {
        assertEquals(2510, file3d.calcularCosto());
    }
}
