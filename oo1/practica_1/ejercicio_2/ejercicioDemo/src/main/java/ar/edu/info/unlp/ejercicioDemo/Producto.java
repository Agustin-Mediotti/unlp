package ar.edu.info.unlp.ejercicioDemo;

public class Producto {
    private double peso;
    private double precioPorKilo;
    private String descripcion;

    public Producto(String descripcion, double peso, double precioPorKilo) {
        this.descripcion = descripcion;
        this.peso = peso;
        this.precioPorKilo = precioPorKilo;
    }

    public double getPrecio() {
        return this.peso * this.precioPorKilo;
    }

    // Getters & Setters

    public double getPeso() {
        return this.peso;
    }

    public double getPrecioPorKilo() {
        return this.precioPorKilo;
    }

    public String getDescripcion() {
        return this.descripcion;
    }

    public void setPeso(double peso) {
        this.peso = peso;
    }

    public void setPrecioPorKilo(double precio) {
        this.precioPorKilo = precio;
    }

    public void setDescripcion(String descripcion) {
        this.descripcion = descripcion;
    }
}
