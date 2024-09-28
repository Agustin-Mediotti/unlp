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
public class Ejercicio5 {
    
    public static void main(String[] args) {
        Partido [] campeonato = new Partido [20];
        Partido partido;
        int cantPartidos = 0, cantRiver=0, cantBoca=0;
         partido = new Partido(Lector.leerString(), Lector.leerString(), Lector.leerInt(), Lector.leerInt());
        while (cantPartidos < 20 && !partido.getVisitante().equals("ZZZ")) {
            campeonato[cantPartidos] = partido;
            System.out.println(partido.getLocal() + ": " + partido.getGolesLocal() + " vs " + partido.getVisitante() + ": " + partido.getGolesVisitante());
            if (partido.getGanador().equalsIgnoreCase("River")) cantRiver++;
            if ((partido.getGanador().equalsIgnoreCase("Boca")) && partido.getLocal().equalsIgnoreCase("Boca")) cantBoca++;
            cantPartidos++;
            partido = new Partido(Lector.leerString(), Lector.leerString(), Lector.leerInt(), Lector.leerInt());
        }
        System.out.println("cantidad de Partidos ganados por River: " + cantRiver);
        System.out.println("cantidad de Partidos ganados por Boca jugando de local: " + cantBoca);
    }
}
