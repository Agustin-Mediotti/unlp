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
public class ProgramadorLider extends Programador {
    private int antiguedad;
    private int cantProyectosDirigidos;

    public ProgramadorLider(String nombre, int dni, double sueldoBasico, int codigoPorHora, String lenguajeDePreferencia, int antiguedad, int cantProyectosDirigidos) {
        super(nombre, dni, sueldoBasico, codigoPorHora, lenguajeDePreferencia);
        this.antiguedad = antiguedad;
        this.cantProyectosDirigidos = cantProyectosDirigidos;
    }

    public int getAntiguedad() {
        return antiguedad;
    }

    public void setAntiguedad(int antiguedad) {
        this.antiguedad = antiguedad;
    }

    public int getCantProyectosDirigidos() {
        return cantProyectosDirigidos;
    }

    public void setCantProyectosDirigidos(int cantProyectosDirigidos) {
        this.cantProyectosDirigidos = cantProyectosDirigidos;
    }
    
    @Override
    public double calcularSueldo() {
        return super.calcularSueldo() + (this.getAntiguedad() * 10000)  + (this.getCantProyectosDirigidos() * 20000);
    }
}
