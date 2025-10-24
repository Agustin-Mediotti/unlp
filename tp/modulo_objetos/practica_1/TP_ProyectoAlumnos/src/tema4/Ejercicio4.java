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

import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author netcreature
 */
public class Ejercicio4 {
    
    private static final int MAXTEMP = 40;
    private static final int MAXMESES = 12;
    
    public static void main(String[] args) {
        EstacionMeterologica e01 = new EstacionMeterologica("La Plata", 34.921, 57.955, 2021, 2024);
        EstacionMeterologica e02 = new EstacionMeterologica("Mar del Plata", 38.002, 57.556, 2020, 2024);
        
        
        for (int i = 0; i < e01.getMaxYear() - e01.getIniYear(); i++) {
            for (int j = 1; j <= MAXMESES; j++) {
                e01.registrarTemperatura(GeneradorAleatorio.generarDouble(MAXTEMP), j, e01.getIniYear()+i);
            }
        }
        System.out.println(e01.generarReporteAnual());
        System.out.println(e01.getMayorTemperatura());
        
        for (int i = 0; i < e02.getMaxYear() - e02.getIniYear(); i++) {
            for (int j = 1; j <= MAXMESES; j++) {
                e02.registrarTemperatura(GeneradorAleatorio.generarDouble(MAXTEMP), j, e02.getIniYear()+i);
            }
        }
        System.out.println(e02.generarReporteMensual());
        System.out.println(e02.getMayorTemperatura());
        
    }
}
