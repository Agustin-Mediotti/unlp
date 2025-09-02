package ar.edu.info.unlp.ejercicioDemo;

public class Balanza {
    private int cantidadDeProductos;
    private double precioTotal;
    private double pesoTotal;

    public Balanza() {
        this.ponerEnCero();
    }

    public void ponerEnCero() {
        this.cantidadDeProductos = 0;
        this.pesoTotal = 0;
        this.precioTotal = 0;
    }

    public void agregarProducto(Producto producto) {
        if (producto == null) {
            throw new IllegalArgumentException("El producto no puede ser null");
        }
        this.pesoTotal += producto.getPeso();
        this.precioTotal += producto.getPrecio();
        this.cantidadDeProductos++;
    }

    public Ticket emitirTicket() {
        return new Ticket(this.cantidadDeProductos, this.precioTotal, this.pesoTotal);
    }

    // Getters

    public int getCantidadDeProductos() {
        return this.cantidadDeProductos;
    }

    public double getPrecioTotal() {
        return this.precioTotal;
    }

    public double getPesoTotal() {
        return this.pesoTotal;
    }

}
