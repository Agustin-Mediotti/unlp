package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class LacerByNTest {

    LacerByN fileDF, fileSF;
    
    @BeforeEach
    void setUp() throws Exception {
        fileDF = new LacerByN(50, true);
        fileSF = new LacerByN(100, false);
    }

    @Test
    public void testCalcularCostoDobleFaz() {
        assertEquals(100, fileDF.calcularCosto());
    }

    @Test
    public void testCalcularCostoSimpleFaz() {
        assertEquals(300, fileSF.calcularCosto());
    }

    @Test
    public void testCalcularDuracion() {
        assertEquals(10, fileSF.calcularDuracion());
        assertEquals(5, fileDF.calcularDuracion());
    }
}
