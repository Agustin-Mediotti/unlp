package edu.unlp;

public class Cuadrado implements Figura2D {
    private double lado;

    public Cuadrado() {
    }

    public void setLado(double lado) {
        this.lado = lado;
    }

    public double getLado() {
        return this.lado;
    }

    public double getArea() {
        return Math.pow(lado, 2);
    }

    public double getPerimetro() {
        return lado * 4;
    }

}
