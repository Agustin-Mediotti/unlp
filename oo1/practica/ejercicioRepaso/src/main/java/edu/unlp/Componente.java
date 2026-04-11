package edu.unlp;

public abstract class Componente {

    protected String nombre;
    protected int cant;

    public Componente(String nombre, int cant) {
        this.nombre = nombre;
        this.cant = cant;
    }

    public abstract double calcularCosto();

    public abstract String generarDescripcion();
}
