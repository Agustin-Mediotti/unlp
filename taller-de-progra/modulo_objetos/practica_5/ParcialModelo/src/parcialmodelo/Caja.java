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
public class Caja {
    private boolean disponible;
    private Ticket[] tickets;
    private int dimL;

    public Caja(int tickets) {
        this.disponible = true;
        this.tickets = new Ticket[tickets];
        this.dimL = 0;
    }

    public boolean isDisponible() {
        return disponible;
    }

    public void setDisponible(boolean disponible) {
        this.disponible = disponible;
    }

    public Ticket[] getTickets() {
        return tickets;
    }

    public void agregarTicket(Ticket ticket) {
        this.tickets[this.dimL] = ticket;
        this.setDimL(this.dimL++);
    }

    public int getDimL() {
        return dimL;
    }

    public void setDimL(int dimL) {
        this.dimL = dimL;
    }
}
