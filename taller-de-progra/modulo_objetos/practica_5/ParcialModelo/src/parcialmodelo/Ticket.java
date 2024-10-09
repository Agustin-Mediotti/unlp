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
public class Ticket {
    private int numeroTicket;
    private int cliente;
    private int librosComprados;
    private double montoAbonado;
    private MedioDePago medioDePago;
   
    
    public Ticket(int numeroTicket, int cliente, int librosComprados, double montoAbonado, MedioDePago medioDePago) {
        this.numeroTicket = numeroTicket;
        this.cliente = cliente;
        this.librosComprados = librosComprados;
        this.montoAbonado = montoAbonado;
        this.medioDePago = medioDePago;
    }

    public int getNumeroTicket() {
        return numeroTicket;
    }

    public void setNumeroTickets(int numeroTickets) {
        this.numeroTicket = numeroTickets;
    }

    public int getCliente() {
        return cliente;
    }

    public void setCliente(int cliente) {
        this.cliente = cliente;
    }

    public int getLibrosComprados() {
        return librosComprados;
    }

    public void setLibrosComprados(int librosComprados) {
        this.librosComprados = librosComprados;
    }

    public double getMontoAbonado() {
        return montoAbonado;
    }

    public void setMontoAbonado(double montoAbonado) {
        this.montoAbonado = montoAbonado;
    }

    public MedioDePago getMedioDePago() {
        return medioDePago;
    }

    public void setMedioDePago(MedioDePago medioDePago) {
        this.medioDePago = medioDePago;
    }
}
