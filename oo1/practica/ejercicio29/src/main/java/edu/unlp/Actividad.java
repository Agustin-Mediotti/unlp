package edu.unlp;

import java.time.LocalDate;

public abstract class Actividad {
    protected LocalDate fechaInicio;
    protected String direccionIp;

    public Actividad(LocalDate fechaInicio, String direccionIp) {
        this.fechaInicio = fechaInicio;
        this.direccionIp = direccionIp;
    }

    public abstract double calcularCosto();
}
