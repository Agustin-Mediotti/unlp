/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;


/**
 *
 * @author netcreature
 */
public class Ejercicio3 {
    
    public static void main(String[] args) {
        Estante estante = new Estante(20);
        Autor tolkien = new Autor("J.R.R. Tolkien");
        
        Libro lotr = new Libro("The Lord of The Rings", "Editorial Minotauro", 2012, tolkien, "SWDSA821", 52.00);
        Libro mujercitas = new Libro("Mujercitas", "Editorial Copa", 2022, tolkien, "SWDSA821", 52.00);
        
        estante.setNuevoLibro(lotr);
        estante.setNuevoLibro(mujercitas);
        
        System.out.println(estante.estaLleno());
        System.out.println(estante.getLibrosAlmacenados());
        
        for(int i = 0; i < estante.getLibrosAlmacenados(); i++) {
            if (estante.getColleccion()[i].getTitulo().equals("Mujercitas")) {
                System.out.println("Autor del libro Mujercitas: " + estante.getColleccion()[i].getPrimerAutor().getNombre());
            }
        }
    }
}
