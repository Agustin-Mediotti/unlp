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
package gestorestacionamiento;

import PaqueteLectura.Lector;

/**
 *
 * @author netcreature
 */
public class GestorEstacionamiento {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Estacionamiento e01 = new Estacionamiento("Zonda", "11 y 43", "08:00", "21:00", 3, 3);
        
        Auto a01 = new Auto("Carlos", "XKA678");
        Auto a02 = new Auto("Felipe", "UAN921");
        Auto a03 = new Auto("Analia", "JUA782");
        Auto a04 = new Auto("Pepe", "OPO168");
        Auto a05 = new Auto("Saul", "NBA420");
        Auto a06 = new Auto("Xuxa", "SXO123");
        
        e01.registrarVehiculo(a01, 0, 0);
        e01.registrarVehiculo(a02, 0, 1);
        e01.registrarVehiculo(a03, 1, 1);
        e01.registrarVehiculo(a04, 1, 0);
        e01.registrarVehiculo(a05, 2, 0);
        e01.registrarVehiculo(a06, 2, 1);
        
        System.out.println(e01.toString());
        System.out.println("Autos en plaza 1: " + e01.autosEnPlaza(1));
        
        System.out.println(e01.buscarVehiculo(Lector.leerString()));
    }
    
}
