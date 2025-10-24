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
public class Estante {
    
    private int dimL;
    private final int dimF;
    private final Libro[] coleccion;
    
    public Estante(int dimF) {
        this.dimF = dimF;
        this.coleccion = new Libro[dimF];
        this.dimL = 0;
    }
    
    public int getLibrosAlmacenados() {
        return this.dimL;
    }
    
    public boolean estaLleno() {
        return this.dimL == this.dimF;
    }
    
    public void setNuevoLibro(Libro nuevoLibro) {
        if (!this.estaLleno()) {
            coleccion[dimL] = nuevoLibro;
            dimL++;
        }
    }
    
    public int getDimF() {
        return this.dimF;
    }

    public Libro[] getColleccion() {
        return this.coleccion;
    }
}
