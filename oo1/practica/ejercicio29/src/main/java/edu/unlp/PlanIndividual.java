package edu.unlp;

public class PlanIndividual implements Plan {

    private int minutosContratados;

    public PlanIndividual(int minutosContratados) {
        this.minutosContratados = minutosContratados;
    }

    @Override
    public double calcularPrecioBase(Cliente cliente) {
        return 20 * minutosContratados;
    }

    @Override
    public double calcularPenalizacion(int ips, int antiguedad) {
        return ips * 300;
    }

}
