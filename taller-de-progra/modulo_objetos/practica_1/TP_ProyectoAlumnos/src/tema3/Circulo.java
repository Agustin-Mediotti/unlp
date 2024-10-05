/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

/**
 *
 * @author netcreature
 */
public class Circulo {
    private double radio;
    private String cRelleno;
    private String cLinea;

    public Circulo(double radio, String cRelleno, String cLinea) {
        this.radio = radio;
        this.cRelleno = cRelleno;
        this.cLinea = cLinea;
    }

    public void setRadio(double radio) {
        this.radio = radio;
    }

    public void setcRelleno(String cRelleno) {
        this.cRelleno = cRelleno;
    }

    public void setcLinea(String cLinea) {
        this.cLinea = cLinea;
    }

    public double getRadio() {
        return radio;
    }

    public String getcRelleno() {
        return cRelleno;
    }

    public String getcLinea() {
        return cLinea;
    }
    
    public double calcularPerimetro() {
        return (2 * Math.PI) * radio;
    }
    
    public double calcularArea() {
        return Math.PI * (Math.pow(radio, 2));
    }
}
