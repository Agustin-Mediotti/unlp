package edu.unlp;

import java.time.LocalDate;
import java.time.LocalTime;
import java.util.ArrayList;
import java.util.List;

public class Solicitud {
    private LocalDate fecha;
    private LocalTime hora;
    private EstadoSolicitud estado;
    private Usuario usuario;
    private List<ITrabajoImpresion> trabajos;

    public Solicitud(Usuario usuario) {
        this.estado = EstadoSolicitud.PENDIENTE;
        this.usuario = usuario;
        this.fecha = LocalDate.now();
        this.hora = LocalTime.now();
        this.trabajos = new ArrayList<ITrabajoImpresion>();
    }

    public void agregarTrabajo(ITrabajoImpresion trabajo) {
        this.trabajos.add(trabajo);
    }

    public String confirmarSolicitud() {
        this.estado = EstadoSolicitud.CONFIRMADO;
        return emitirComprobante(usuario.getCredito(), usuario.descontarCredito(trabajos.stream().mapToDouble(trabajo -> trabajo.calcularCosto()).sum()));
    }

    private String emitirComprobante(double montoCubierto, double saldoAPagar) {
        return "Usuario: " + usuario.getDni() + " - Fecha: " + fecha + "\n" + 
            "Trabajos incluidos:" + "\n" + trabajos.stream().map(trabajo -> trabajo.toString()).reduce("", String::concat) + 
            "Tiempo estimado de realización: " + trabajos.stream().mapToDouble(trabajo -> trabajo.calcularDuracion()).sum() + "\n" +
            "Costo cubierto por crédito: $" + montoCubierto + "\n" +
            "Saldo a pagar: $" + saldoAPagar;
    }

    public LocalDate getFecha() {
        return fecha;
    }

    public LocalTime getHora() {
        return hora;
    }

    public Usuario getUsuario() {
        return usuario;
    }

    public EstadoSolicitud getEstado() {
        return estado;
    }

    public List<ITrabajoImpresion> getTrabajos() {
        return trabajos;
    }

}
