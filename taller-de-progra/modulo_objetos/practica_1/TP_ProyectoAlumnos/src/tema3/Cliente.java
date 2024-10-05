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
    private String nombre;
    private int DNI;
    private int edad;

    public Cliente(String nombre, int DNI, int edad) {
        this.nombre = nombre;
        this.DNI = DNI;
        this.edad = edad;
    }

    public String getNombre() {
        return nombre;
    }

    public int getDNI() {
        return DNI;
    }

    public int getEdad() {
        return edad;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public void setDNI(int DNI) {
        this.DNI = DNI;
    }

    public void setEdad(int edad) {
        this.edad = edad;
    }
    
}