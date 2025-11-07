package edu.unlp;

public class LacerByN implements ITrabajoImpresion {

    private boolean dobleFaz;
    private int paginas;

    public LacerByN(int paginas, boolean dobleFaz) {
        this.paginas = paginas;
        this.dobleFaz = dobleFaz;
    }

    @Override
    public double calcularCosto() {
        return paginas * (dobleFaz ? 2 : 3); 
    }

    @Override
    public double calcularDuracion() {
        return paginas * 0.1;
    }

    @Override
    public String toString() {
        return "Impresion BN, " + paginas + " pag. " + (dobleFaz ? "Doble Faz" : "Simple Faz") + "\n";
    }

}
