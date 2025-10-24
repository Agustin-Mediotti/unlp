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
public class Jugador extends Empleado {
    private int partidosJugados;
    private int golesAnotados;
    
    public Jugador(String nombre, int antiguedad, int partidosJugados, int golesAnotados ) {
        super(nombre, antiguedad);
        this.partidosJugados = partidosJugados;
        this.golesAnotados = golesAnotados;
    }

    public int getPartidosJugados() {
        return partidosJugados;
    }

    public void setPartidosJugados(int partidosJugados) {
        this.partidosJugados = partidosJugados;
    }

    public int getGolesAnotados() {
        return golesAnotados;
    }

    public void setGolesAnotados(int golesAnotados) {
        this.golesAnotados = golesAnotados;
    }
    
    @Override
    public double calcularEfectividad() {
        return this.getPartidosJugados() / this.getGolesAnotados();
    }
    
    @Override
    public double calcularSueldoACobrar() {
        return this.calcularEfectividad() > 0.5 ? this.getSueldoBasico() * 2 : this.getSueldoBasico();
    }
}
