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
public class Hotel {
    private final int maxHabitaciones;
    private final Habitacion[] habitaciones;

    public Hotel(int maxHabitaciones) {
        this.maxHabitaciones = maxHabitaciones;
        this.habitaciones = new Habitacion[maxHabitaciones];
        
        for (int i = 0; i < maxHabitaciones; i++) {
            habitaciones[i] = new Habitacion();
        }
    }

    public int getMaxHabitaciones() {
        return maxHabitaciones;
    }

    public Habitacion[] getHabitaciones() {
        return this.habitaciones;
    }

    public void setNuevoPrecio(double precio) {
        for (Habitacion habitacion : habitaciones) {
            habitacion.setCostoPorNoche(precio);
        }
    }
    
    public String toString() {
        String aux="";
        for (int i = 0; i < maxHabitaciones; i++) {
            String reserva = habitaciones[i].isOcupada() ? " Ocupada. Cliente: " + habitaciones[i].getReserva().getNombre() : "Libre ";
            aux += "Habitacion " + i + " $" + habitaciones[i].getCostoPorNoche() + " " + reserva;
        }
        return aux;
    }
    
    public void setNuevaReserva(Cliente nuevoCliente, int habitacion) {
        this.habitaciones[habitacion].setReserva(nuevoCliente);
    }
}
