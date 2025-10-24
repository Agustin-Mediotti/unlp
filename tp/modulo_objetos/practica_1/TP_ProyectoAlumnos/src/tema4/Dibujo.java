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
public class Dibujo {
    private String titulo;
    private Figura [] vector;
    private int guardadas;
    private final int capacidadMaxima=10;
    
    //inicia el dibujo, sin figuras
    public Dibujo (String titulo){
        this.titulo = titulo;
        this.vector = new Figura[capacidadMaxima];
    }
    
    //agrega la figura al dibujo
    public void agregar(Figura f){
        this.vector[guardadas] = f;
        this.guardadas++;
        System.out.println("la figura " + f.toString() + " se ha guardado" + "\n");
    }
    
    //calcula el área del dibujo:
    //suma de las áreas de sus figuras
    public double calcularArea(){
        double total=0;
        for(int i=0; i < this.guardadas; i++) {
            total =+ vector[i].calcularArea();
        }
        return total;
    }
    
    //imprime el título, representación
    //de cada figura, y área del dibujo
    public void mostrar(){
        String figuras = "";
        for(int i=0; i < this.guardadas; i++) {
            figuras = figuras.concat("\t" + this.vector[i].toString() + "\n");
        }
        
        System.out.println(this.titulo + ":" + "\n" + figuras + " Area Total: " + this.calcularArea());
    }
    
    //retorna está lleno el dibujo
    public boolean estaLleno() {
        return (guardadas == capacidadMaxima);
    }
}
