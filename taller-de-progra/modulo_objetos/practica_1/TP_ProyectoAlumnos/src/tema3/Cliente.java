/*
 * NOTAS: Reúse la clase Persona. Para cada método solicitado piense a qué clase debe
 * delegar la responsabilidad de la operación.
 */
package tema3;

import tema2.Persona;

/**
 *
 * @author Alumno
 */
public class Cliente extends Persona {

    public Cliente(String nombre, int DNI, int edad) {
        this.nombre = nombre;
        this.DNI = DNI;
        this.edad = edad;
    }
    
}