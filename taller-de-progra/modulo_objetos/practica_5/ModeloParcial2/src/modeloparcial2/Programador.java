/*
 * Copyright 2024 netcreature.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package modeloparcial2;

/**
 *
 * @author netcreature
 */
public class Programador {
    private final int ADICIONAL = 50000;
    
    private String nombre;
    private int dni;
    private double sueldoBasico;
    private int codigoPorHora;
    private String lenguajeDePreferencia;

    public Programador(String nombre, int dni, double sueldoBasico, int codigoPorHora, String lenguajeDePreferencia) {
        this.nombre = nombre;
        this.dni = dni;
        this.sueldoBasico = sueldoBasico;
        this.codigoPorHora = codigoPorHora;
        this.lenguajeDePreferencia = lenguajeDePreferencia;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getDni() {
        return dni;
    }

    public void setDni(int dni) {
        this.dni = dni;
    }

    public double getSueldoBasico() {
        return sueldoBasico;
    }

    public void setSueldoBasico(double sueldoBasico) {
        this.sueldoBasico = sueldoBasico;
    }

    public int getCodigoPorHora() {
        return codigoPorHora;
    }

    public void setCodigoPorHora(int codigoPorHora) {
        this.codigoPorHora = codigoPorHora;
    }

    public String getLenguajeDePreferencia() {
        return lenguajeDePreferencia;
    }

    public void setLenguajeDePreferencia(String lenguajeDePreferencia) {
        this.lenguajeDePreferencia = lenguajeDePreferencia;
    }
    
    public double calcularSueldo() {
        if (this.getCodigoPorHora() > 200) {
            return this.sueldoBasico + ADICIONAL;
        } else {
            return this.sueldoBasico;
        }        
    }
    
    public void aumentarSueldo(double aumento) {
        this.setSueldoBasico(this.getSueldoBasico() + aumento);
    }
    
    @Override
    public String toString() {
        return  " Nombre: " + this.getNombre() + " DNI: " + this.getDni() + " Lenguaje: "+ this.getLenguajeDePreferencia() + " Sueldo final: " + this.calcularSueldo();
    }
}
