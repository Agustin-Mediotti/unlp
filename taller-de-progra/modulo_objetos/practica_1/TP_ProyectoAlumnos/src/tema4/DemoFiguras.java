/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema4;


public class DemoFiguras {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Cuadrado cuad = new Cuadrado(10,"Rojo", "Negro");
        System.out.println("Color linea: " + cuad.getColorLinea()); 
        System.out.println("Area: " + cuad.calcularArea()); 
        System.out.println("Representacion del cuadrado: " + cuad.toString());
        
        Triangulo tri = new Triangulo("Rojo", "Verde", 5, 6, 3);
        System.out.println(tri.toString());
        tri.despintar();
        System.out.println(tri.toString());
        
        System.out.println();
        
        Circulo cir = new Circulo("Rojo", "Verde", 5);
        System.out.println(cir.toString());
        cir.despintar();
        System.out.println(cir.toString());
    }
    
    
    
}
