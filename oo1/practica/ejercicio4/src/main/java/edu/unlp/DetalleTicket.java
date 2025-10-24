package edu.unlp;

public class DetalleTicket {

    private String descripcion;
    private double peso;
    private double precio;
    private double precioPorKilo;

    public DetalleTicket(Producto producto) {
        this.descripcion = producto.getDescripcion();
        this.peso = producto.getPeso();
        this.precio = producto.getPrecio();
        this.precioPorKilo = producto.getPrecioPorKilo();
    }

    // Getters

    public String getDescripcion() {
        return descripcion;
    }

    public double getPeso() {
        return peso;
    }

    public double getPrecio() {
        return precio;
    }

    public double getPrecioPorKilo() {
        return precioPorKilo;
    }
}
