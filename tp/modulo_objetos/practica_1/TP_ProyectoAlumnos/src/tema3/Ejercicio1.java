/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

import PaqueteLectura.Lector;

/**
 *
 * @author netcreature
 */
public class Ejercicio1 {
    public static void main(String[] args) {
        
        System.out.println("LADO1, LADO2, LADO3, RELLENO, LINEA");
        Triangulo tri = new Triangulo(Lector.leerDouble(), Lector.leerDouble(), Lector.leerDouble(), Lector.leerString(), Lector.leerString());
        
        System.out.println("Perimetro: " + tri.calcularPerimetro() + " Area: " + tri.calcularArea());
    }
}
