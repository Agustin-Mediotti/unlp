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
public class Ejercicio5 {
    public static void main(String[] args) {
        Circulo circ = new Circulo(30, "RED", "GREEN");
        
        System.out.println("Perimetro: " + circ.calcularPerimetro());
        System.out.println("Perimetro: " + circ.calcularArea());
    }
}
