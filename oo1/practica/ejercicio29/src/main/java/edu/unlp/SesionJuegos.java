package edu.unlp;

import java.time.LocalDate;
import java.util.ArrayList;

public class SesionJuegos extends Actividad {
    private int duracion;
    private ArrayList<Item> items;

    public SesionJuegos(LocalDate fechaInicio, String direccionIp, int duracion) {
        super(fechaInicio, direccionIp);
        this.duracion = duracion;
    }

    public void agregarItem(Item item) {
        this.items.add(item);
    }

    @Override
    public double calcularCosto() {
        if (this.duracion > 360) {
            return items.stream().mapToDouble(item -> item.calcularAdicional()).sum();
        } else {
            return 0;
        }
    }

}
