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
public class Ejercicio3B {
    
    public static void main(String[] args) {
        Persona jorge = new Persona("Jorge", 38253264, 30);
        Trabajador fede = new Trabajador("Federico", 41127496, 26, "Electricista");
        
        System.out.println(jorge.toString());
        System.out.println(fede.toString());
    }
}
