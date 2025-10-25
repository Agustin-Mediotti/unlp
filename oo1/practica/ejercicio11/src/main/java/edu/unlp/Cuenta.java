package edu.unlp;

public abstract class Cuenta {
    /*
     * Saldo tendría que ser de tipo long o BigDecimal 
     * para evitar errores de Redondeo
     */
    private double saldo;

    public Cuenta() {
        this.saldo = 0;
    }

    public double getSaldo() {
        return this.saldo;
    }

    public void depositar(double monto) {
        this.saldo += monto;
    }

    protected void extraerSinControlar(double monto) {
        this.saldo -= monto;
    }

    public boolean extraer(double monto) {
        if (this.puedeExtraerMonto(monto)) {
            this.extraerSinControlar(monto);
            return true;
        }
        return false;
    }

    public boolean transferirACuenta(double monto, Cuenta cuentaDestino) {
        if(this.puedeExtraerMonto(monto)) {
            this.extraerSinControlar(monto);
            cuentaDestino.depositar(monto);
            return true;
        }
        return false;
    }

    protected boolean puedeExtraerMonto(double monto) {
        return this.saldo >= monto;
    }
}
