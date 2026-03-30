package edu.unlp;

public interface Plan {
    public abstract double calcularPrecioBase(Cliente cliente);

    public abstract double calcularPenalizacion(int ips, int antiguedad);
}
