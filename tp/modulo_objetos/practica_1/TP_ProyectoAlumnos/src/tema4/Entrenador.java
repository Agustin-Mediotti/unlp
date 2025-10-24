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
package tema4;

/**
 *
 * @author netcreature
 */
public class Entrenador extends Empleado {
    private int campeonatosGanados;
    
    public Entrenador(String nombre, int antiguedad, int campeonatosGanados) {
        super(nombre, antiguedad);
        this.campeonatosGanados = campeonatosGanados;
    }

    public int getCampeonatosGanados() {
        return campeonatosGanados;
    }

    public void setCampeonatosGanados(int campeonatosGanados) {
        this.campeonatosGanados = campeonatosGanados;
    }

    @Override
    public double calcularEfectividad() {
        return this.getAntiguedad() / this.getCampeonatosGanados();
    }
    
    @Override
    public double calcularSueldoACobrar() {
        
        switch((1 <= this.getCampeonatosGanados() && this.getCampeonatosGanados() <= 4) ? 0 : 
                (5 <= this.getCampeonatosGanados() && this.getCampeonatosGanados() <= 10) ? 1 : 2) {
            case 0: return this.getSueldoBasico()+5000;
            case 1: return this.getSueldoBasico()+30000;
            case 2: return this.getSueldoBasico()+50000;
            default: return this.getSueldoBasico();
        }
    }
}
