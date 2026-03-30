package edu.unlp;

import java.time.LocalDate;
import java.time.Period;
import java.util.ArrayList;
import java.util.HashSet;

public class Cliente {
    private String nombre;
    private LocalDate fechaAlta;
    private Plan plan;
    private ArrayList<Actividad> actividades;
    private HashSet<String> ips;

    public Cliente(String nombre, LocalDate fecha, Plan plan) {
        this.nombre = nombre;
        this.fechaAlta = fecha;
        this.plan = plan;
    }

    public String getNombre() {
        return this.nombre;
    }

    public void cambiarPlan(Plan plan) {
        this.plan = plan;
    }

    public int calcularAntiguedad() {
        return Period.between(fechaAlta, LocalDate.now()).getYears();
    }

    public double montoTotalACobrar(LocalDate fechaInicio, LocalDate fechaFin) {
        return plan.calcularPrecioBase(this) + plan.calcularPenalizacion(cantidadIps(), calcularAntiguedad());
    }

    public int cantidadIps() {
        actividades.stream().forEach(actividad -> ips.add(actividad.direccionIp));
        return ips.size();
    }
}
