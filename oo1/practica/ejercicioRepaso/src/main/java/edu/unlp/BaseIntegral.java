package edu.unlp;

public class BaseIntegral extends Componente {

    public BaseIntegral(String tipoBase, int cantPorciones) {
        super(tipoBase, cantPorciones);
    }

    @Override
    public double calcularCosto() {
        return 2200;
    }

    @Override
    public String generarDescripcion() {
        return "Base de " + nombre + " (integral, " + cant + " porciones)";
    }
}
