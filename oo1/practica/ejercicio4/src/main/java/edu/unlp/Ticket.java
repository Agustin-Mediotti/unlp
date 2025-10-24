package edu.unlp;

import java.time.LocalDate;
import java.util.stream.Collectors;
import java.util.List;

public class Ticket {
    private LocalDate fecha;
    private List<DetalleTicket> productos;

    public Ticket(List<Producto> productos) {
        this.fecha = LocalDate.now();
        this.productos = productos.stream().map(DetalleTicket::new).collect(Collectors.toList());
    }

    public double impuesto() {
        return this.getPrecioTotal() * 0.21;
    }

    // Getters

    public int getCantidadDeProductos() {
        return productos.size();
    }

    public double getPesoTotal() {
        double total = 0;
        for (DetalleTicket producto : this.productos) {
            total += producto.getPeso();
        }
        return total;
    }

    public double getPrecioTotal() {
        double total = 0;
        for (DetalleTicket producto : this.productos) {
            total += producto.getPrecio();
        }
        return total;
    }

    public LocalDate getFecha() {
        return this.fecha;
    }

    public List<DetalleTicket> getProductos() {
        return this.productos;
    }
}
