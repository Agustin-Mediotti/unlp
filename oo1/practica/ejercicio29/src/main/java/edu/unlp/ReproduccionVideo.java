package edu.unlp;

import java.time.LocalDate;

public class ReproduccionVideo extends Actividad {

    private int duracionTotal;
    private int duracionAds;

    public ReproduccionVideo(LocalDate fechaInicio, String direccionIp, int duracionTotal, int duracionAds) {
        super(fechaInicio, direccionIp);
        this.duracionAds = duracionAds;
        this.duracionTotal = duracionTotal;
    }

    @Override
    public double calcularCosto() {
        return duracionReal() * 10;
    }

    public int duracionReal() {
        return duracionTotal - duracionAds;
    }

}
