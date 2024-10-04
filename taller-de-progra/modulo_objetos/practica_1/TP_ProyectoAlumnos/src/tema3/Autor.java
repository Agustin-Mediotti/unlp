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
public class Autor {
    
    private String nombre;
    private String biografia;
    private String origen;

    public Autor(String nombre, String biografia, String origen) {
        this.nombre = nombre;
        this.biografia = biografia;
        this.origen = origen;
    }
    
    public Autor(String nombre) {
        this.nombre = nombre;
        this.biografia = "Vestibulum feugiat consequat semper. Sed ut erat id nisl varius suscipit. "
                + "Morbi faucibus a velit ac tincidunt";
        this.origen = "Aenean";
    }

    public String getNombre() {
        return this.nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getBiografia() {
        return this.biografia;
    }

    public void setBiografia(String biografia) {
        this.biografia = biografia;
    }

    public String getOrigen() {
        return this.origen;
    }

    public void setOrigen(String origen) {
        this.origen = origen;
    }
    
    @Override
    public String toString() { 
        return "Nombre: " + this.nombre + "Bio: " + this.biografia + "Origen: " + this.origen;
    }

    boolean getName() {
        throw new UnsupportedOperationException("Not supported yet."); //To change body of generated methods, choose Tools | Templates.
    }
    
}
