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
public class GestorProyectosUNLP {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Proyecto p01 = new Proyecto(1273819234, "MOB1", "Dr. Guillermo Santa María");
        
        Investigador inv01 = new Investigador("Lic. Martin Miguel de Mueles", 5, "Biotecnologia Naranja");
        Investigador inv02 = new Investigador("Dra. Juliana Zurduy", 2, "Cultivo Microbiologico y Bio Computadores");
        Investigador inv03 = new Investigador("Dr. Juan Manuel de Orquidea", 1, "Sistemas Confocales Microscopicos");
        
        p01.agregarInvestigador(inv01);
        p01.agregarInvestigador(inv02);
        p01.agregarInvestigador(inv03);
        
        Subsidio sub01 = new Subsidio(400_000.00, "Estudio de los migrasomas del parásito Trichomonas vaginalis: una nueva organela involucrada en migración");
        Subsidio sub02 = new Subsidio(780_000.00, "Estudio de la coordinación endocrina y neuroendocrina del control de la ingesta y el eje de crecimiento somático en peces");
        Subsidio sub03 = new Subsidio(1_250_000.00, "Agroquímicos en lagunas de la región pampeana. Efectos de la atrazina y del glifosato sobre la capacidad reproductiva.");
        Subsidio sub04 = new Subsidio(380_000.00, "Rol de la proteína MOB1 en la división celular del parásito Tritrichomonas foetus.");
        Subsidio sub05 = new Subsidio(750_000.00, "El desafío de mejorar la eficiencia de los bioformulados de uso agrícola. Una aproximación basada en el uso de consorcios microbianos.");
        Subsidio sub06 = new Subsidio(490_500.00, "Optimización de pruebas serológicas para el diagnóstico de la toxoplasmosis.");
        
        inv01.agregarSubsidio(sub01);
        inv01.agregarSubsidio(sub02);
        
        inv02.agregarSubsidio(sub03);
        inv02.agregarSubsidio(sub04);
        
        inv03.agregarSubsidio(sub05);
        inv03.agregarSubsidio(sub06);
        
        p01.otorgarTodos("Dra. Juliana Zurduy");
        System.out.println(p01.toString() + "\n" + inv02.getSubsidios()[1].getMotivo());
    }
    
}
