package edu.unlp;

import java.util.ArrayList;
import java.util.List;

public class Farola {

    private boolean estado;
    private List<Farola> neightbors;

    public Farola() {
        this.estado = false;
        this.neightbors = new ArrayList<Farola>();
    }

    public List<Farola> getNeighbors() {
        return this.neightbors;
    }

    public boolean isOn() {
        return estado;
    }

    public boolean isOff() {
        return !estado;
    }

    public void turnOn() {
        if(!this.estado) {
            this.estado = true;
            for (Farola neightbor : this.neightbors) neightbor.turnOn();
        }
    }

    public void turnOff() {
        if (this.estado) {
            this.estado = false;
            for (Farola neightbor : this.neightbors) neightbor.turnOff();
        }
    }

    public boolean isNeightbor(Farola farola) {
        for(Farola element : this.neightbors) {
            if (element == farola) {
                return true;
            }
        }
        return false;
    }

    public void pairWithNeighbor(Farola otraFarola) {
        this.neightbors.add(otraFarola);
        if (!otraFarola.isNeightbor(this)) otraFarola.pairWithNeighbor(this);
    }

}
