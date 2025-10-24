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
package gestorproyectosunlp;

/**
 *
 * @author netcreature
 */
public class Investigador {
    private String nombre;
    private int categoria;
    private String especialidad;
    private final Subsidio[] subsidios;
    private int cantSubsidios;
    
    public Investigador(String nombre, int categoria,String especialidad) {
        this.nombre= nombre;
        this.especialidad = especialidad;
        this.categoria =  categoria > 5 ? 0 : categoria;
        this.subsidios = new Subsidio[5];
        this.cantSubsidios = 0;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public int getCategoria() {
        return categoria;
    }

    public void setCategoria(int categoria) {
        this.categoria = categoria;
    }

    public String getEspecialidad() {
        return especialidad;
    }

    public void setEspecialidad(String especialidad) {
        this.especialidad = especialidad;
    }

    public int getCantSubsidios() {
        return cantSubsidios;
    }

    public void setCantSubsidios(int cantSubsidios) {
        this.cantSubsidios = cantSubsidios;
    }
    
    public Subsidio[] getSubsidios() {
        return this.subsidios;
    }
    
    public void agregarSubsidio(Subsidio subsidio) {
        this.subsidios[this.cantSubsidios] = subsidio;
        this.cantSubsidios++;
    }
}
