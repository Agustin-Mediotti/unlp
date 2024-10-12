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
public class Empresa {
    private String nombre;
    private ProgramadorLider lider;
    private final Programador[] equipo;
    private int cantEquipo;

    public Empresa(String nombre, ProgramadorLider lider, int maxEquipo) {
        this.nombre = nombre;
        this.lider = lider;
        this.equipo = new Programador[maxEquipo];
        this.cantEquipo = 0;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public ProgramadorLider getLider() {
        return lider;
    }

    public void setLider(ProgramadorLider lider) {
        this.lider = lider;
    }

    public Programador[] getEquipo() {
        return equipo;
    }

    public int getCantEquipo() {
        return cantEquipo;
    }

    public void setCantEquipo(int cantEquipo) {
        this.cantEquipo = cantEquipo;
    }
    
    public void agregarProgramador(Programador programador) {
        this.equipo[cantEquipo] = programador;
        this.cantEquipo++;
    }
    
    public void aumentarSueldos(double monto) {
        for (Programador pj : this.getEquipo())
            if (pj != null) pj.aumentarSueldo(monto);
    }
    
    @Override
    public String toString() {
        String data = ""; double total = 0;
        for (int i = 0; i < this.getCantEquipo(); i++) {
            data += ("\t" + "Programador " + (i+1) + ": " + this.equipo[i].toString() + "\n");
            total+= this.equipo[i].calcularSueldo();
        }
        return "Empresa " + this.getNombre() + "\n"
                + "Programador Lider: " + this.getLider().toString() + "\n"
                + data + "\n" 
                + "Monto total en sueldos a abonar: $" + (total + this.getLider().calcularSueldo()) + "\n";
    }
}
