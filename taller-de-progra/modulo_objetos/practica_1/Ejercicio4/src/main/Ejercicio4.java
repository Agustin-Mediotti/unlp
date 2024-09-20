/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package main;

import PaqueteLectura.Lector;

/**
 *
 * @author netcreature
 */
public class Ejercicio4 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        int [][] edificio = new int [8][4];
        int piso, ofi, i, j;
        
        /**
         *  Inicializamos la matriz
         */
        
        for (i = 0; i < 8; i++) {
            for (j = 0; j < 4; j++) {
                edificio[i][j] = 0;
            }
        }
        
        
        /**
         * Cargamos los datos
         */
        
        piso = Lector.leerInt();
        ofi = Lector.leerInt();
        while (piso != 9) {
            edificio[piso][ofi]+=1; 
            piso = Lector.leerInt();
            if (piso != 9) ofi = Lector.leerInt();
        }
        
        /**
         * Informamos lo pedido xd
         */
        
        for (i=0; i<8; i++) {
            for (j=0; j<4; j++) {
                System.out.println("En la oficina " + j + " del piso " + i + " concurrieron: " + edificio[i][j] + " personas.");
            }
        }
        
    }
    
}
