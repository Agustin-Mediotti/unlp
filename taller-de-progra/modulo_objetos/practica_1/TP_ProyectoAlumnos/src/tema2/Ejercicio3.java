
package tema2;


import PaqueteLectura.Lector;
/**
 *
 * @author netcreature
 */
public class Ejercicio3 {
   
    
    public static Persona leerPersona(Persona persona) {
        System.out.println("Ingresar DNI, EDAD, NOMBRE");
        persona.setDNI(Lector.leerInt());
        persona.setEdad(Lector.leerInt());
        persona.setNombre(Lector.leerString());
        return persona;
    }
    
    public static void main(String[] args) {
        Persona [][] casting = new Persona[5][8];
        Persona persona = new Persona();
        
        int dia = 0; int turno; int i,j;
        
        
    persona = leerPersona(persona);
        while(dia < 5) {
            turno = 0;
            while (turno < 8 && !persona.getNombre().equals("ZZZ")) {
                casting[dia][turno] = persona;
                turno++;
                persona = leerPersona(persona);
            }
            dia++;
        }

       // inciso b
        for (i = 0; i < 5; i++) {
            for (j = 0; j< 8; j++) {
                if (casting[i][j] != null) System.out.println("dia: " + (i+1) + " turno: " + (j+1) + " nombre: " + casting[i][j].getNombre());
            } 
        }
    }
    
}
