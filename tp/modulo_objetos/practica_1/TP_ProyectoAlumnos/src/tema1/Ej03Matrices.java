/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema1;

//Paso 1. importar la funcionalidad para generar datos aleatorios
import PaqueteLectura.GeneradorAleatorio;


public class Ej03Matrices {

    public static void main(String[] args) {
	//Paso 2. iniciar el generador aleatorio     
	 GeneradorAleatorio.iniciar();
        //Paso 3. definir la matriz de enteros de 5x5 e iniciarla con nros. aleatorios 
         int[][] matrix  = new int [5] [5];
         int i,j;
         for (i = 0; i < 5; i++) {
             for (j = 0; j < 5; j++) {
                 matrix[i][j] = GeneradorAleatorio.generarInt(200);
             }
         }
        //Paso 4. mostrar el contenido de la matriz en consola
         for (i = 0; i < 5; i++) {
             System.out.println();
             System.out.println("--------------------------");
             for (j = 0; j < 5; j++) {
                 System.out.print(matrix[i][j] + " | ");
             }
             System.out.println();
             System.out.println("--------------------------");
         }
        //Paso 5. calcular e informar la suma de los elementos de la fila 1
         System.out.println();
         
         int sumFila = 0;
         for (i = 0; i< 5; i++) sumFila+= matrix[1][i];
         
         System.out.println("Suma de los elementos de la fila 1: " + sumFila);
    
        //Paso 6. generar un vector de 5 posiciones donde cada posición j contiene la suma de los elementos de la columna j de la matriz. 
        //        Luego, imprima el vector.
         int [] vec = new int [5];
         int sum = 0;
         for (i=0; i<5;i++) {
             for(j=0; j<5;j++) {
                sum += matrix[i][j];
                vec[j] = sum;
             }
         }
         
         for (i=0; i<5; i++) System.out.println(vec[i]);

        //Paso 7. lea un valor entero e indique si se encuentra o no en la matriz. En caso de encontrarse indique su ubicación (fila y columna)
        //   y en caso contrario imprima "No se encontró el elemento".
        int dato = PaqueteLectura.Lector.leerInt();
        for (i = 0; i < 5; i++) {
            for (j = 0; j < 5; j++) {
                if (matrix[i][j] == dato) System.out.println("Numero encontrado en posicion: Fila: ["+ i +"]"+" Columna: ["+ j +"]");
            }
        }
    }
}
