/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package ejercicio5;

import PaqueteLectura.Lector;

/**
 *
 * @author Agustin Mediotti
 */
public class Ejercicio5 {
    
    static int PERSONAS = 2;
    static int CATEGORIES = 4;
    
    public enum Categories {
        ATENCION_AL_CLIENTE, CALIDAD_DE_LA_COMIDA, PRECIO, AMBIENTE
    }

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
        int i,j;
        int [] promedios = new int [CATEGORIES];
        int [][] califications = new int [PERSONAS][CATEGORIES];
        
        
        for (i=0; i<PERSONAS; i++) {
            
            System.out.println("Cliente Numero: " + i);
            
            for (j=0; j<CATEGORIES; j++) {
                
                switch(j) {
                    case 0: System.out.println(Categories.ATENCION_AL_CLIENTE.toString());
                        break;
                    case 1: System.out.println(Categories.CALIDAD_DE_LA_COMIDA.toString());
                        break;
                    case 2: System.out.println(Categories.PRECIO.toString());
                        break;
                    case 3: System.out.println(Categories.AMBIENTE.toString());
                        break;
                }
                califications[i][j] = Lector.leerInt();
            }
        }
        
        for (i=0; i<PERSONAS; i++) {
            for (j=0; j<CATEGORIES; j++) {
                promedios[j] += califications[i][j];
            }
        }
        
        for (j=0; j<CATEGORIES; j++) System.out.println("Promedio en Categoria " + j + ": " + promedios[j]/PERSONAS);
    }
    
}
