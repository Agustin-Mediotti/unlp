package edu.unlp;

import java.util.Date;

public class Mamifero {
    private String identificador;
    private String especie;
    private Date fechaNacimiento;
    private Mamifero padre;
    private Mamifero madre;

    public Mamifero() {
        this.madre = null;
        this.padre = null;
    }

    public Mamifero(String identificador) {
        this.identificador = identificador;
        this.madre = null;
        this.padre = null;
    }

    public String getIdentificador() {
        return identificador;
    }

    public void setIdentificador(String identificador) {
        this.identificador = identificador;
    }

    public String getEspecie() {
        return especie;
    }

    public void setEspecie(String especie) {
        this.especie = especie;
    }

    public Date getFechaNacimiento() {
        return fechaNacimiento;
    }

    public void setFechaNacimiento(Date fechaNacimiento) {
        this.fechaNacimiento = fechaNacimiento;
    }

    public Mamifero getPadre() {
        return padre;
    }

    public void setPadre(Mamifero padre) {
        this.padre = padre;
    }

    public Mamifero getMadre() {
        return madre;
    }

    public void setMadre(Mamifero madre) {
        this.madre = madre;
    }

    public Mamifero getAbueloMaterno() {
        if (this.madre != null) {
            return this.madre.getPadre();
        } else {
            return null;
        }
    }

    public Mamifero getAbuelaMaterna() {
        if (this.madre != null) {
            return this.madre.getMadre();
        } else {
            return null;
        }
    }

    public Mamifero getAbueloPaterno() {
        if (this.padre != null) {
            return this.padre.getPadre();
        } else {
            return null;
        }
    }

    public Mamifero getAbuelaPaterna() {
        if (this.padre != null) {
            return this.padre.getMadre();
        } else {
            return null;
        }
    }

    public boolean tieneComoAncestroA(Mamifero unMamifero) { 
        if (this.padre != null && this.padre.identificador == unMamifero.identificador) {
            return true;
        }
        if (this.madre != null && this.madre.identificador  == unMamifero.identificador) {
            return true;
        }
        
        if (this.padre != null && this.padre.tieneComoAncestroA(unMamifero)) {
            return true;
        }

        if (this.madre != null && this.madre.tieneComoAncestroA(unMamifero)) {
            return true;
        }

        return false;
    }
}
