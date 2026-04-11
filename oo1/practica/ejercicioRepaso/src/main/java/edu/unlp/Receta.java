package edu.unlp;

import java.util.ArrayList;

public class Receta {
    private String nombre;
    private ArrayList<Componente> componentes;

    public Receta(String nombre) {
        this.nombre = nombre;
        this.componentes = new ArrayList<Componente>();
    }

    public String getNombre() {
        return this.nombre;
    }

    public String generarDescripcion() {
        return componentes.stream().map(c -> c.generarDescripcion() + "\n").toString();
    }

    public double costoEstimado() {
        return componentes.stream().mapToDouble(c -> c.calcularCosto()).sum();
    }
}
