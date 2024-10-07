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

import tema2.Persona;

/**
 *
 * @author netcreature
 */
public class Trabajador extends Persona {
    
    private String oficio;
    
    public Trabajador(String nombre, int dni, int edad, String oficio) {
        super(nombre, dni, edad);
        this.oficio = oficio;
    }

    public String getOficio() {
        return oficio;
    }

    public void setOficio(String oficio) {
        this.oficio = oficio;
    }
    
    @Override
    public String toString() {
        return "Mi nombre es " + nombre + ", mi DNI es " + DNI + " y tengo " + edad + " años. Soy " + this.getOficio();
    }
}
