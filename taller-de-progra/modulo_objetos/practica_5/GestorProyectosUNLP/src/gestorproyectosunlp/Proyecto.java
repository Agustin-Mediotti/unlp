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

import java.util.Arrays;

/**
 *
 * @author netcreature
 */
public class Proyecto {
    private static final int MAXINVESTIGADORES = 50;
    
    private int codigo;
    private String nombre;
    private String nombreDirector;
    private final Investigador[] investigadores;
    private int cantInvestigadores;
    
    public Proyecto(int codigo, String nombre, String nombreDirector) {
        this.investigadores = new Investigador[50];
        this.codigo = codigo;
        this.nombre = nombre;
        this.nombreDirector = nombreDirector;
        this.cantInvestigadores = 0;
    }

    public int getCodigo() {
        return codigo;
    }

    public void setCodigo(int codigo) {
        this.codigo = codigo;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getNombreDirector() {
        return nombreDirector;
    }

    public void setNombreDirector(String nombreDirector) {
        this.nombreDirector = nombreDirector;
    }

    public Investigador[] getInvestigadores() {
        return investigadores;
    }
    
    public void agregarInvestigador(Investigador investigador) {
        this.investigadores[cantInvestigadores] = investigador;
        this.cantInvestigadores++;
    }
    
    public double dineroTotalOtorgado() {
        double total=0;
        for(int i=0; i < this.cantInvestigadores; i++) {
            for(int j=0; j < this.investigadores[i].getCantSubsidios(); j++) {
                if(this.investigadores[i].getSubsidios()[j].isOtorgado()) total += this.investigadores[i].getSubsidios()[j].getMontoPedido();
            }
        }
        return total;
    }
    
    public void otorgarTodos(String nombre) {
        for(int i=0; i< this.cantInvestigadores; i++) {
            if(this.getInvestigadores()[i].getNombre().equals(nombre)) {
                for(Subsidio sub : this.getInvestigadores()[i].getSubsidios()) {
                    if(sub != null) sub.setOtorgado(true);
                }
            }
        }
    }
    
    @Override
    public String toString() {
        String datos="";
        for(int i=0; i < this.cantInvestigadores; i++) {
            double cant = 0;
            for(int j=0; j < this.getInvestigadores()[i].getCantSubsidios(); j++) {
                if (this.getInvestigadores()[i].getSubsidios()[j].isOtorgado()) cant += this.getInvestigadores()[i].getSubsidios()[j].getMontoPedido();
            }
            datos = datos.concat("\t" + "- " + this.getInvestigadores()[i].getNombre() 
                    + " " + this.getInvestigadores()[i].getCategoria() + " " 
                    + this.getInvestigadores()[i].getEspecialidad() + " $" + cant + "\n");
        }
        
        return "Proyecto: " + this.nombre + " codigo: " + this.codigo + "\n" 
                + "Director del proyecto: " + this.nombreDirector + "\n"
                + "Total dinero otorgado al proyecto: $" + this.dineroTotalOtorgado() + "\n"
                + "\n" + "Investigadores:" + "\n"
                + datos;
    }
}
