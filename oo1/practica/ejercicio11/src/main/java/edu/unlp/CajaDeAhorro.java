package edu.unlp;

public class CajaDeAhorro extends Cuenta {

    @Override
    public boolean extraer(double monto) {
        monto += calcularAdicional(monto);
        if(super.puedeExtraerMonto(monto)) {
            extraerSinControlar(monto);
            return true;
        }
        return false;
    }

    @Override
    public void depositar(double monto) {
        monto -= calcularAdicional(monto);
        super.depositar(monto);
    }

    
    @Override
    public boolean transferirACuenta(double monto, Cuenta cuentaDestino) {
        double nuevoMonto = monto + calcularAdicional(monto);
        if (puedeExtraerMonto(nuevoMonto)) {
            cuentaDestino.depositar(monto);
            this.extraerSinControlar(nuevoMonto);
            return true;
        }
        return false;
    }

    private double calcularAdicional(double monto) {
        return monto * 2 / 100;
    }
}
