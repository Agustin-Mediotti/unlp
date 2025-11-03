package edu.unlp;

import java.util.ArrayList;
import java.util.List;

public class Inversor {
    private List<Inversion> inversiones;

    public Inversor() {
        this.inversiones = new ArrayList<>();
    }

    public double getMontoInversionTotal() {
        return inversiones.stream()
        .mapToDouble(inversion -> inversion.calcularValorActual()).sum();
    }

    public void agregarInversion(Inversion inversion) {
        inversiones.add(inversion);
    }

    public void removerInversion(Inversion inversion) {
        inversiones.remove(inversion);
    }

    public int getCantidadInversiones() {
        return inversiones.size();
    }
}
