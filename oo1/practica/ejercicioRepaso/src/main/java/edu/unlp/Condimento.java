package edu.unlp;

public class Condimento extends Componente {
    private boolean picante;

    public Condimento(String menzclaCondimentos, int cantCucharitas, boolean picante) {
        super(menzclaCondimentos, cantCucharitas);
        this.picante = picante;
    }

    @Override
    public double calcularCosto() {
        return 0;
    }

    @Override
    public String generarDescripcion() {
        return "Condimento " + nombre + (picante ? " (picante " : " (no picante") +
                cant + " cucharaditas)";
    }

}
