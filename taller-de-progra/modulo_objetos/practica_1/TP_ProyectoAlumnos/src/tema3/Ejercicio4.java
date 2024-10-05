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
public class Ejercicio4 {
    public static void main(String[] args) {
        Hotel california = new Hotel(5);
        Cliente nuevoCliente = new Cliente("Vitto Agustin Mediotti", 39509979, 28);
        
        california.setNuevaReserva(nuevoCliente, 3);
        System.out.println(california.getHabitaciones()[3].isOcupada());
        System.out.println(california.toString());
    }
}
