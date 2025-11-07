package edu.unlp;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class SolicitudTest {
    
    private Usuario user;
    private Solicitud solicitud;
    private ITrabajoImpresion trabajoLacer;
    private ITrabajoImpresion trabajo3D;
    
    @BeforeEach
    void setUp() throws Exception {
        user = new Usuario("John Doe", 19201928, 1000);
        solicitud = new Solicitud(user);
        trabajoLacer = new LacerByN(100, false);
        solicitud.agregarTrabajo(trabajoLacer);
    }

    @Test
    public void testAgregarTrabajo() {
        trabajo3D = new Impresion3D("file3d.gcode", 500);
        solicitud.agregarTrabajo(trabajo3D);
        assertEquals(2, solicitud.getTrabajos().size());
        assertEquals(trabajo3D, solicitud.getTrabajos().toArray()[1]);
    }

    @Test
    public void testConfirmarSolicitud() {
        assertEquals("Usuario: " + user.getDni() +  " - Fecha: " + solicitud.getFecha().toString() + "\n" +
            "Trabajos incluidos:" + "\n" + trabajoLacer.toString() +
            "Tiempo estimado de realización: "
            + trabajoLacer.calcularDuracion() + "\n" +
            "Costo cubierto por crédito: $" + user.getCredito() + "\n" +
            "Saldo a pagar: $" + (trabajoLacer.calcularCosto() < user.getCredito() ? 0 : trabajoLacer.calcularCosto() - user.getCredito())
        , solicitud.confirmarSolicitud());
    }
}
