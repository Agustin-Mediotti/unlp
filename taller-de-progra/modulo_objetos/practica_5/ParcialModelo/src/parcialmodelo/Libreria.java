/*
 * Copyright 2024 netcreature.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package parcialmodelo;

/**
 *
 * @author netcreature
 */
public class Libreria {
    private String nombre;
    private int numeroVentas;
    private Caja[] cajas;

    public Libreria(String nombre) {
        this.nombre = nombre;
        this.numeroVentas = 0;
        this.cajas = new Caja[4];
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getNumeroVentas() {
        return numeroVentas;
    }

    public void setNumeroVentas(int numeroVentas) {
        this.numeroVentas = numeroVentas;
    }

    public Caja[] getCajas() {
        return cajas;
    }

    public void setCajas(Caja[] cajas) {
        this.cajas = cajas;
    }
    
    public Ticket generarTicket(int caja, int cliente, int cantidadLibros, double monto, MedioDePago medio) {
        Ticket ticket = new Ticket(this.numeroVentas, cliente, cantidadLibros, monto, medio);
        this.setNumeroVentas(this.numeroVentas++);
        this.getCajas()[caja].agregarTicket(ticket);
        return ticket;
    }
    
    public void actualizarCajas(int cantMinima) {
        for(int i = 0; i < 4; i++) {
            if(this.getCajas()[i].getDimL() < cantMinima)
                this.getCajas()[i].setDisponible(false);
        }
    }
    
    public Ticket obtenerVentaMaxima() {
        double max = 0; Ticket ventaMaxima = null;
        for(int i = 0; i < 4; i++) {
            for(int j = 0; j< this.getCajas()[i].getDimL(); j++) {
                if(this.getCajas()[i].getTickets()[j].getMontoAbonado() > max) {
                    max = this.getCajas()[i].getTickets()[j].getMontoAbonado();
                    ventaMaxima =  this.getCajas()[i].getTickets()[j];
                }
            }
        }
        return ventaMaxima;
    }
    
    @Override
    public String toString() {
        String data = "";
        for(int i = 0; i < 4; i++) {
            data = data.concat("Caja " + (i+1) + ": " + String.valueOf(this.cajas[i].isDisponible()) + " " + String.valueOf(this.getCajas()[i].getDimL()));
            for(Ticket ticket : this.getCajas()[i].getTickets()) {
                data = data.concat("\t" + ticket.getNumeroTicket() + " " 
                        + ticket.getCliente() + " " + ticket.getLibrosComprados() + " " + String.valueOf(ticket.getMontoAbonado()) + " " + ticket.getMedioDePago() );
            }
        }
        return data;
    } 
} 
