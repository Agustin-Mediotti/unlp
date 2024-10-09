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

/**
 *
 * @author netcreature
 */
public class Estacionamiento {
    
    private String nombre;
    private String direccion;
    private String horaApertura;
    private String horaCierre;
    private Auto[][] lugares;
    private int pisos = 5;
    private int plazas = 10;

    public Estacionamiento(String nombre, String direccion) {
        this.nombre = nombre;
        this.direccion = direccion;
        this.horaApertura = "08:00hs";
        this.horaCierre = "21:00hs";
        this.lugares = new Auto[pisos][plazas];
    }
    
    public Estacionamiento(String nombre, String direccion, String horaApertura, String horaCierre, int pisos, int plazas) {
        this.nombre = nombre;
        this.direccion = direccion;
        this.horaApertura = horaApertura;
        this.horaCierre = horaCierre;
        this.lugares = new Auto[pisos][plazas];
        this.pisos = pisos;
        this.plazas = plazas;
    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getDireccion() {
        return direccion;
    }

    public void setDireccion(String direccion) {
        this.direccion = direccion;
    }

    public String getHoraApertura() {
        return horaApertura;
    }

    public void setHoraApertura(String horaApertura) {
        this.horaApertura = horaApertura;
    }

    public String getHoraCierre() {
        return horaCierre;
    }

    public void setHoraCierre(String horaCierre) {
        this.horaCierre = horaCierre;
    }

    public Auto[][] getLugares() {
        return lugares;
    }

    public void setLugares(Auto[][] lugares) {
        this.lugares = lugares;
    }
    
    public int getPisos() {
        return pisos;
    }

    public void setPisos(int pisos) {
        this.pisos = pisos;
    }

    public int getPlazas() {
        return plazas;
    }

    public void setPlazas(int plazas) {
        this.plazas = plazas;
    }
    
    public void registrarVehiculo(Auto auto, int piso, int plaza) {
        this.lugares[piso][plaza] = auto;
    }
    
    public String buscarVehiculo(String patente) {
        for(int i = 0; i < this.getPisos(); i++) {
            for(int j = 0; j < this.getPlazas(); j++) {
                if(this.lugares[i][j]!=null && this.lugares[i][j].getPatente().equals(patente))
                    return "Vehiculo en Piso: " + i + " Plaza: " + j;
            }
        }
        return "Auto inexistente";
    }
    
    public int autosEnPlaza(int plaza) {
        int total = 0;
        for(int i = 0; i < this.getPisos(); i++) {
            if (this.lugares[i][plaza] != null)
                total+=1;
        }
        return total;
    }
    
    @Override
    public String toString() {
        String data = "";
        for(int i = 0; i < this.getPisos(); i++) {
            for(int j = 0; j < this.getPlazas(); j++) {
                data = this.lugares[i][j] != null ? data.concat("\t" + "Piso: " + i + " Plaza: " + j + " Vehiculo: " + this.lugares[i][j].toString() + "\n") : data.concat("\t" + "Libre" + "\n");
            }
        }
        return data;
    }
}
