package edu.unlp;

public class Impresion3D implements ITrabajoImpresion {

    private double gramos;
    private String fileName;

    public Impresion3D(String fileName, double gramos) {
        this.fileName = fileName;
        this.gramos = gramos;
    }

    @Override
    public double calcularCosto() {
        return 5 * gramos + (2 * calcularDuracion());
    }

    @Override
    public double calcularDuracion() {
        return gramos / 100;
    }

    @Override
    public String toString() {
        return "Impresion3D, " + fileName + " " + gramos + "gr. " + "\n";
    }

    public String getFileName() {
        return fileName;
    }

}
