
package tema1;

//Paso 1: Importar la funcionalidad para lectura de datos
import PaqueteLectura.Lector;

public class Ej02Jugadores {

  
    public static void main(String[] args) {
        //Paso 2: Declarar la variable vector de double 
        double [] vec;
        //Paso 3: Crear el vector para 15 double 
        vec = new double[15];
        //Paso 4: Declarar indice y variables auxiliares a usar
         int i, cant = 0;
         double aux = 0;
        //Paso 6: Ingresar 15 numeros (altura), cargarlos en el vector, ir calculando la suma de alturas
        for (i = 0; i < 15; i++) {
            vec[i] = Lector.leerDouble();
            aux = aux + vec[i];
        }
        //Paso 7: Calcular el promedio de alturas, informarlo
        System.out.println("Promedio de alturas: " + aux/15);
        //Paso 8: Recorrer el vector calculando lo pedido (cant. alturas que están por encima del promedio)
        for (i = 0; i < 15; i++) {
            if (vec[i] > aux/15)
                cant++;
        }
        //Paso 9: Informar la cantidad.
        System.out.println("Cant. alturas por encima del promedio: " + cant);
    }
    
}
