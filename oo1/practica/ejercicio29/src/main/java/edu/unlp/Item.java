package edu.unlp;

public class Item {
    private String nombre;
    private int cantidad;
    private double precioUnitario;

    public Item(String nombre, int cantidad, double precioUnitario) {
        this.nombre = nombre;
        this.cantidad = cantidad;
        this.precioUnitario = precioUnitario;
    }

    public String getNombre() {
        return nombre;
    }

    public double calcularAdicional() {
        return cantidad * precioUnitario;
    }
}
