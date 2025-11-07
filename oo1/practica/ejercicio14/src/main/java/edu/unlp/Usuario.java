package edu.unlp;

public class Usuario {
    private String nombre;
    private int dni;
    private double credito;

    public Usuario(String nombre, int dni, double credito) {
        this.nombre = nombre;
        this.dni = dni;
        this.credito = credito;
    }

    public String getNombre() {
        return nombre;
    }

    public int getDni() {
        return dni;
    }

    public double getCredito() {
        return credito;
    }

    public double descontarCredito(double credito) {
        if (credito <= this.credito) {
            this.credito -= credito;
            return 0;
        } else {
            double aPagar = credito - this.credito;
            this.credito = 0;
            return aPagar;
        }
    }
}
