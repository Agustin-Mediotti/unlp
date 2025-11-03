package edu.unlp;

import java.time.LocalDate;
import java.time.temporal.ChronoUnit;

public class IPlazoFijo implements Inversion {

    private double valorInicial;
    private double tasaInteres;
    private LocalDate fechaInicial;

    public IPlazoFijo(double valorInicial, double tasaInteres) {
        this.fechaInicial = LocalDate.now();
        this.valorInicial = valorInicial;
        this.tasaInteres = tasaInteres;
    }

    @Override
    public double calcularValorActual() {
        return valorInicial + ((valorInicial * tasaInteres / 100) * ChronoUnit.DAYS.between(fechaInicial, LocalDate.now()));
    }
}
