package tema2;

import PaqueteLectura.Lector;

/**
 *
 * @author netcreature
 */
public class Ejercicio1 {
    
    public static void main (String[] args ) {
        Persona persona = new Persona();
        
        persona.setNombre(Lector.leerString());
        persona.setDNI(Lector.leerInt());
        persona.setEdad(Lector.leerInt());
        
        System.out.println(persona.toString());
    }
}
