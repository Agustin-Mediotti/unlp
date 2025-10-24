/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;

/**
 *
 * @author netcreature
 */
public class Circulo extends Figura {
    
    private double radio;

    public Circulo(String unCR, String unCL, double unRadio) {
        super(unCR, unCL);
        this.radio = unRadio;
    }

    @Override
    public double calcularArea() {
        return Math.pow(Math.PI * radio, 2);
    }

    @Override
    public double calcularPerimetro() {
        return 2 * Math.PI * this.getRadio();
    }

    public double getRadio() {
        return radio;
    }

    public void setRadio(double radio) {
        this.radio = radio;
    }
    
    
}
