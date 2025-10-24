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
public class Ejercicio2 {
    
    public static void main(String[] args) {
        Jugador lioMessi = new Jugador("Lionel Messi", 20, 1200, 800);
        Entrenador zubeldia = new Entrenador("Osvaldo Zubeldia", 20, 8);
        
        System.out.println(lioMessi.toString());
        System.out.println(zubeldia.toString());
    }
}
