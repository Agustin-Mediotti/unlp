package edu.unlp;

public class PlanGrupal implements Plan {

    private int limiteIps;

    public PlanGrupal(int limiteIps) {
        this.limiteIps = limiteIps;
    }

    @Override
    public double calcularPrecioBase(Cliente cliente) {
        return 800 * limiteIps;
    }

    @Override
    public double calcularPenalizacion(int ips, int antiguedad) {
        if (ips > limiteIps & antiguedad < 10) {
            return (ips - limiteIps) * 500;
        } else {
            return 0;
        }
    }

}
