/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;

import PaqueteLectura.Lector;

/**
 *
 * @author netcreature
 */
public class Ejercicio4 {
    
    public static void leerPersona(Persona persona) {
        System.out.println("DNI, EDAD Y NOMBRE");
        persona.setDNI(Lector.leerInt());
        persona.setEdad(Lector.leerInt());
        persona.setNombre(Lector.leerString());
    }
    
    public static void main(String [] args) {
        Persona[][] casting = new Persona[5][8];
        Persona actorNuevo = new Persona();
        int i,j,dia,turno,cant,total = 0;
        
        leerPersona(actorNuevo);
        while (total < 40 && !actorNuevo.getNombre().equals("ZZZ")) {
            System.out.println("DIA Y TURNO:");
            dia = Lector.leerInt();
            turno = Lector.leerInt();
            if (casting[dia][turno] != null) {
                System.out.println("El turno ya esta ocupado.");
            } else {
                casting[dia][turno] = actorNuevo;
                total++;
                
                System.out.println("Turno reservado.");
            }
            leerPersona(actorNuevo);
        }
        // inciso b
        for(i = 0; i < 5; i++) {
            cant=0;
            for (j=0;j<8;j++) {
                if (casting[i][j] != null) {
                    System.out.println("Actor: " + casting[i][j].getNombre());
                    cant++;
                }
            }
            System.out.println("cantidad de actores en el dia " + i + ": " + cant);
        }
        
    }
}
