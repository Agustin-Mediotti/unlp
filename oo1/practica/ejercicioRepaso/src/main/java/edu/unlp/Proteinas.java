package edu.unlp;

public class Proteinas extends Componente {
    private String formaPresentacion;
    private double precioPorcion;

    public Proteinas(String tipoProte, String formaPresentacion, int cantPorciones, double precioPorcion) {
        super(tipoProte, cantPorciones);
        this.formaPresentacion = formaPresentacion;
        this.precioPorcion = precioPorcion;
    }

    @Override
    public double calcularCosto() {
        return precioPorcion * cant;
    }

    @Override
    public String generarDescripcion() {
        return "Proteína de " + nombre + " en " + formaPresentacion
                + "(" + cant + " porciones a $" + precioPorcion + " por porción)";
    }

}
