package edu.unlp;

public class BaseTradicional extends Componente {

    public BaseTradicional(String tipoBase, int cantPorciones) {
        super(tipoBase, cantPorciones);
    }

    @Override
    public double calcularCosto() {
        return 1500;
    }

    @Override
    public String generarDescripcion() {
        return "Base de " + nombre + "(tradicional, " + cant + " porciones)";
    }

}
