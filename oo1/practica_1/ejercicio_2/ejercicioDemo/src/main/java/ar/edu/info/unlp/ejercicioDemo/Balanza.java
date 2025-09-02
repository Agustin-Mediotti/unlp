package ar.edu.info.unlp.ejercicioDemo;

public class Balanza {
    private int cantidadDeProductos;
    private double precioTotal;
    private double pesoTotal;

    public Balanza() {
        this.cantidadDeProductos = 0;
        this.pesoTotal = 0;
        this.precioTotal = 0;
    }

    public void ponerEnCero() {
        this.cantidadDeProductos = 0;
        this.pesoTotal = 0;
        this.precioTotal = 0;
    }

    public void agregarProducto(Producto producto) {
        this.pesoTotal += producto.getPeso();
        this.precioTotal += producto.getPrecio();
        this.cantidadDeProductos++;
    }

    public Ticket emitirTicket() {
        return new Ticket(this.cantidadDeProductos, this.precioTotal, this.pesoTotal);
    }

    // Getters & Setters

    public void setCantidadDeProductos(int cant) {
        this.cantidadDeProductos = cant;
    }

    public void setPrecioTotal(double precio) {
        this.precioTotal = precio;
    }

    public void setPesoTotal(double peso) {
        this.pesoTotal = peso;
    }

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
