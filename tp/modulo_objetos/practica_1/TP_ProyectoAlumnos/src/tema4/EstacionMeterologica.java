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
public class EstacionMeterologica {
    
    private static final int MINVALUE = 8000;
    private static final int MAXVALUE = 9000;
    
    private String name;
    private double lat;
    private double lon;
    private int maxYear;
    private int iniYear;
    private int month;
    private final double[][] temp;
    
    private static enum Meses {
        Enero,
        Febrero,
        Marzo,
        Abril,
        Mayo,
        Junio,
        Julio,
        Agosto,
        Septiembre,
        Octubre,
        Noviembre,
        Diciembre
    }
    
    public EstacionMeterologica(String name, double lat, double lon, int iniYear, int maxYear) {
        this.name = name;
        this.lat = lat;
        this.lon = lon;
        this.maxYear = maxYear;
        this.iniYear = iniYear;
        
        this.temp = new double[maxYear - iniYear][12];
        
        
        for (int i = 0; i < maxYear - iniYear; i++) {
            for (int j = 0; j < 12; j++) {
                double aux = 0;
                while (aux < MINVALUE) {
                    aux = GeneradorAleatorio.generarDouble(MAXVALUE);
                }
                this.temp[i][j] = aux;
            }
        }
    }
    
    public void registrarTemperatura(double temp, int month, int year) {
        this.temp[year - this.iniYear][month-1] = temp;
    }
    
    public double getTemperatura(int month, int year) {
        return this.temp[year - this.iniYear][month-1];
    }
    
    public String getMayorTemperatura() {
        int yyyy = 0, mm = 0; double max = 0;
        
        for (int i = 0; i < maxYear - iniYear; i++) {
            for (int j = 0; j < 12; j++) {
                if (temp[i][j] > max) {
                    max = this.temp[i][j];
                    yyyy = i;
                    mm = j;
                }
            }
        }
        
        return "El mes " + (mm+1) + " del año " + (yyyy + this.iniYear) + " registro una temperatura de " + max + "°C" + "\n";
    }
    
    public String generarReporteAnual() {
        String data = ""; double promedio;
        
        for (int i = 0; i < maxYear - iniYear; i++) {
            promedio = 0;
            for (int j = 0; j < 12; j++) {
                promedio =+ this.temp[i][j];
            }
           data = data.concat("\t" + "- Año " + (i + iniYear) + ": " + promedio + "°C" + "\n");
        }
        
        
        return this.name + " (" + this.lat + " S - " + this.lon + " O):" + " \n" + data;
    }
    
        public String generarReporteMensual() {
        String data = ""; double[] promedio = new double[12]; Meses[] meses = Meses.values();
        
        for (int i = 0; i < 12 ; i++) {
            promedio[i] = 0;
            for (int j = 0; j < this.maxYear - this.iniYear; j++) {
                promedio[i] =+ this.temp[j][i];
            }
            promedio[i] = promedio[i] / (this.maxYear - this.iniYear);
            data = data.concat("\t" + "-" + meses[i] + ": " + promedio[i] + "°C" + "\n");
        }
        
        
        return this.name + " (" + this.lat + " S - " + this.lon + " O):" + " \n" + data;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public double getLat() {
        return lat;
    }

    public void setLat(double lat) {
        this.lat = lat;
    }

    public double getLon() {
        return lon;
    }

    public void setLon(double lon) {
        this.lon = lon;
    }

    public int getMaxYear() {
        return maxYear;
    }

    public void setMaxYear(int maxYear) {
        this.maxYear = maxYear;
    }

    public int getIniYear() {
        return iniYear;
    }

    public void setIniYear(int iniYear) {
        this.iniYear = iniYear;
    }

    public int getMonth() {
        return month;
    }

    public void setMonth(int month) {
        this.month = month;
    }

    public double[][] getTemperaturas() {
        return temp;
    }
    
    public void printTable() {
        for (int i = 0; i < this.maxYear - this.iniYear; i++) {
            for (int j=0; j<12; j++) {
                System.out.println(temp[i][j]);
            }
        }
    }
    
}
