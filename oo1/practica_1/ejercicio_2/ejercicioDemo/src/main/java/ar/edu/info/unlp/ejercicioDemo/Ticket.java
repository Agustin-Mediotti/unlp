package ar.edu.info.unlp.ejercicioDemo;

import java.time.LocalDate;

public class Ticket {
    private LocalDate fecha;
    private int cantidadDeProductos;
    private double pesoTotal;
    private double precioTotal;

    public Ticket(int cantidad, double precioTotal, double pesoTotal) {
        this.fecha = LocalDate.now();
        this.cantidadDeProductos = cantidad;
        this.pesoTotal = pesoTotal;
        this.precioTotal = precioTotal;
    }

    public double impuesto() {
        return this.precioTotal * 0.21;
    }

    public void setCantidadDeProductos(int cantidad) {
        this.cantidadDeProductos = cantidad;
    }

    public void setPesoTotal(double peso) {
        this.pesoTotal = peso;
    }

    public void setPrecioTotal(double precio) {
        this.precioTotal = precio;
    }

    public int getCantidadDeProductos() {
        return this.cantidadDeProductos;
    }

    public double getPesoTotal() {
        return this.pesoTotal;
    }

    public double getPrecioTotal() {
        return this.precioTotal;
    }

    public LocalDate getFecha() {
        return this.fecha;
    }
}
