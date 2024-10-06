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
public class Triangulo extends Figura {
    private double ladoA;
    private double ladoB;
    private double ladoC;

    public Triangulo(String unCR, String unCL, double ladoA, double ladoB, double ladoC) {
        super(unCR, unCL);
        this.ladoA = ladoA;
        this.ladoB = ladoB;
        this.ladoC = ladoC;
    }

    @Override
    public double calcularArea() {
        return (Math.sqrt(Math.pow((this.getLadoA()), 2) - Math.pow((this.getLadoB()), 2)) * this.getLadoB()) / 2;
    }

    @Override
    public double calcularPerimetro() {
        return this.getLadoA() + this.getLadoB() + this.getLadoC();
    }
    
    @Override
    public String toString() {
        return "Lado A, B y C: " + this.getLadoA() + " " + this.getLadoB() + " " + this.getLadoC() +
                " Area: " + this.calcularArea() + " Perimetro: " + this.calcularPerimetro();
    }

    public double getLadoA() {
        return ladoA;
    }

    public void setLadoA(double ladoA) {
        this.ladoA = ladoA;
    }

    public double getLadoB() {
        return ladoB;
    }

    public void setLadoB(double ladoB) {
        this.ladoB = ladoB;
    }

    public double getLadoC() {
        return ladoC;
    }

    public void setLadoC(double ladoC) {
        this.ladoC = ladoC;
    }
    
    
    
}
