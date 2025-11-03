package edu.unlp;

import java.util.List;

public class IAcciones implements Inversion {
    private String nombre;
    private int cantAcciones;
    private List<Accion> acciones;
    private double precioPorAccion;


    public IAcciones(String nombre, double precioPorAccion, List<Accion> acciones) {
        this.nombre = nombre;
        this.cantAcciones = acciones.size();
        this.acciones = acciones;
        this.precioPorAccion = precioPorAccion;
    }

    @Override
    public double calcularValorActual() {
        return acciones.size() * precioPorAccion;
    }

    public String getNombre() {
        return nombre;
    }
    
    public int getCantAcciones() {
        return cantAcciones;
    }

}
