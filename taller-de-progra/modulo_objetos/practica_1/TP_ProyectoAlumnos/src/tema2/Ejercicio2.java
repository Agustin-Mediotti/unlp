package tema2;

import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author netcreature
 */
public class Ejercicio2 {
    
    public static void main(String[] args) {
        int edad,cant,dni,i = 0;
        Persona minimoDNI = null;
        Persona[] personas = new Persona[15];
        GeneradorAleatorio.iniciar();
        
        edad = GeneradorAleatorio.generarInt(100);
        while (edad != 0 && i < 15 ) {
            personas[i] = new Persona(GeneradorAleatorio.generarString(4), GeneradorAleatorio.generarInt(9), edad);
            edad = GeneradorAleatorio.generarInt(100);
            i++;
        }
        cant=0; dni=999999999;
        for (Persona persona : personas) {
            if (persona.getEdad() > 65) cant+=1;
            if (persona.getDNI() < dni) {
                dni = persona.getDNI();
                minimoDNI = persona;
            }
        }
        
        System.out.println("Personas mayores a 65: " + cant);
        if (minimoDNI != null) System.out.println("Persona con menor DNI: " + minimoDNI.toString());
        
    }
}
