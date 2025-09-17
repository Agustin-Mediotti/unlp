package edu.unlp;

import java.time.LocalDate;
import java.util.ArrayList;

public class Presupuesto {
    private LocalDate fecha;
    private String cliente;
    private ArrayList<Item> items;

    public Presupuesto(String cliente) {
        this.cliente = cliente;
        this.fecha = LocalDate.now();
        this.items = new ArrayList<Item>();
    }

    public void agregarItem(Item item) {
        if (item != null) {
            this.items.add(item);
        }
    }

    public double calcularTotal() {
        return this.items.stream().mapToDouble(item->item.costo()).sum();
    }

    public String getCliente() {
        return this.cliente;
    }

    public LocalDate getFecha() {
        return this.fecha;
    }
}
