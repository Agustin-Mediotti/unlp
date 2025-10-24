package edu.unlp;

public class Circulo implements Figura2D {
    private double radio;

    public Circulo() {
    }

    public double getDiametro() {
        return this.getRadio() * 2;
    }

    public void setDiametro(double diametro) {
        this.radio = diametro * 0.5;
    }

    public double getRadio() {
        return this.radio;
    }

    public void setRadio(double radio) {
        this.radio = radio;
    }

    public double getPerimetro() {
        return Math.PI * this.getDiametro();
    }

    public double getArea() {
        return Math.PI * Math.pow(this.getRadio(), 2);
    }

}
