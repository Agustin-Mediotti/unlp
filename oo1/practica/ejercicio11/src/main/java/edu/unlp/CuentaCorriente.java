package edu.unlp;

public class CuentaCorriente extends Cuenta {
    
    private double descubierto;
    private double limite;

    public CuentaCorriente() {
        this.descubierto = 0;
        this.limite = 0;
    }

    @Override
    public boolean extraer(double monto) {
        if (monto <= this.getSaldo()) {
            return super.extraer(monto);
        } else if (monto <= this.getSaldo() + (this.getLimite() - this.getDescubierto())) {
            this.setDescubierto(monto - this.getSaldo());
            this.depositar(monto - this.getSaldo());
            return super.extraer(monto);
        }
        return false;
    }

    public double getDescubierto() {
        return descubierto;
    }

    public void setDescubierto(double descubierto) {
        this.descubierto = descubierto;
    }

    public double getLimite() {
        return limite;
    }

    public void setLimite(double limite) {
        this.limite = limite;
    }
}
