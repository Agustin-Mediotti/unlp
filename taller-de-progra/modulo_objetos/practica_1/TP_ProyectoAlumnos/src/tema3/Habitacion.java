/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author netcreature
 */
public class Habitacion {
    private double costoPorNoche;
    private boolean ocupada;
    private Cliente reserva;
    
    
    public Habitacion() {
        this.ocupada = false;
        this.reserva = null;
        
        while(this.costoPorNoche < 2000) {
            this.costoPorNoche = GeneradorAleatorio.generarDouble(8000);
        }
    }

    public boolean isOcupada() {
        return ocupada;
    }

    public void setReserva(Cliente cliente) {
        this.ocupada = true;
        this.reserva = cliente;
    }

    public Cliente getReserva() {
        return reserva;
    }
    
    public void setCostoPorNoche(double precio) {
        this.costoPorNoche = precio;
    }
}
