package edu.unlp;

public class Cuerpo3D {
    private double altura;
    private Figura2D caraBasal;

    public Cuerpo3D() {

    }

    public void setAltura(double valor) {
        this.altura = valor;
    }

    public double getAltura() {
        return this.altura;
    }

    public void setCaraBasal(Figura2D cara) {
        this.caraBasal = cara;
    }

    public double getVolumen() {
        return this.caraBasal.getArea() * altura;
    }

    public double getSuperficieExterior() {
        return this.caraBasal.getArea() * 2 + this.caraBasal.getPerimetro() * altura;
    }

    public double getArea() {
        return this.caraBasal.getArea() * altura;
    }
}
