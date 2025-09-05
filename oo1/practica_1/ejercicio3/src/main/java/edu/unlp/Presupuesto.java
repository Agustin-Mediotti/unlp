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
        if (this.items == null) {
            return 0;
        }

        double total = 0;
        for (Item item : this.items) {
            total += item.costo();
        }
        return total;
    }

    public String getCliente() {
        return this.cliente;
    }

    public LocalDate getFecha() {
        return this.fecha;
    }
}
