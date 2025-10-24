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
public class ModeloParcial2 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        ProgramadorLider pepe = new ProgramadorLider("Pepe", 124142, 500000, 300, "PHP", 5, 10);
        Empresa bigDeal = new Empresa("Big Deal", pepe, 3);
        
        Programador jojo = new Programador("jojos", 167235, 200000, 200, "Perl");
        Programador emilia = new Programador("emilia", 287645621, 150000, 150, "Clojure");
        
        bigDeal.agregarProgramador(jojo);
        bigDeal.agregarProgramador(emilia);
        
        System.out.println(bigDeal.toString());
        
        bigDeal.aumentarSueldos(45120);
        
        System.out.println(bigDeal.toString());
    }
    
}
