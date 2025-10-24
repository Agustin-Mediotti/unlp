package edu.unlp;

public class Item {
    private String detalle;
    private int cantidad;
    private double costoUnitario;

    public Item(String detalle, int cantidad, double costoUnitario) {
        this.detalle = detalle;
        this.cantidad = cantidad;
        this.costoUnitario = costoUnitario;
    }

    public double costo() {
        return costoUnitario * cantidad;
    }

    public String getDetalle() {
        return this.detalle;
    }

    public double getCostoUnitario() {
        return this.costoUnitario;
    }
}
