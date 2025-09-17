package edu.unlp;

import java.util.ArrayList;
import java.util.List;

public class Balanza {
    private List<Producto> productos;

    public Balanza() {
        this.productos = new ArrayList<Producto>();
    }

    public void ponerEnCero() {
        this.productos.clear();
    }

    public void agregarProducto(Producto producto) {
        if (producto == null) {
            throw new IllegalArgumentException("El producto no puede ser null");
        }
        this.productos.add(producto);
    }

    public Ticket emitirTicket() {
        return new Ticket(this.productos);
    }

    // Getters

    public int getCantidadDeProductos() {
        return this.productos.size();
    }

    public double getPrecioTotal() {
        return this.productos.stream().mapToDouble(producto->producto.getPrecio()).sum();
    }

    public double getPesoTotal() {
        return this.productos.stream().mapToDouble(producto->producto.getPeso()).sum();
    }

    public List<Producto> getProductos() {
        return this.productos;
    }

}
