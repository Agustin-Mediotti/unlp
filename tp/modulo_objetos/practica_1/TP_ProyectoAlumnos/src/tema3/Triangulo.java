/*

    1-A- Definir una clase para representar triángulos. Un triángulo se caracteriza por el
    tamaño de sus 3 lados (double), el color de relleno (String) y el color de línea (String).
    Provea un constructor que reciba todos los datos necesarios para iniciar el objeto.
    Provea métodos para:
        - Devolver/modificar el valor de cada uno de sus atributos (métodos get y set)
        - Calcular el perímetro y devolverlo (método calcularPerimetro)
        - Calcular el área y devolverla (método calcularArea)
    Devolver/modificar el valor de cada uno de sus atributos (métodos get y set)
    Calcular el perímetro y devolverlo (método calcularPerimetro)
    Calcular el área y devolverla (método calcularArea)
    B- Realizar un programa que instancie un triángulo, le cargue información leída desde
    teclado e informe en consola el perímetro y el área.
    NOTA: Calcular el área con la fórmula Á𝑟𝑒𝑎 = 𝑠(𝑠 − 𝑎)(𝑠 − 𝑏)(𝑠 − 𝑐) , donde a, b y c
    𝑎+𝑏+𝑐
    son los lados y 𝑠 = 2 . La función raíz cuadrada es Math.sqrt(#)

 */

package tema3;

/**
 *
 * @author netcreature
 */
public class Triangulo {
    private double lado1;
    private double lado2;
    private double lado3;
    private String colorRelleno;
    private String colorLinea;
    
    public Triangulo(double lado1, double lado2, double lado3, String colorRelleno, String colorLinea) {
        this.lado1 = lado1;
        this.lado2 = lado2;
        this.lado3 = lado3;
        this.colorRelleno = colorRelleno;
        this.colorLinea = colorLinea;
    }
    
    // GETTERS

    public double getLado1() {
        return lado1;
    }

    public double getLado2() {
        return lado2;
    }

    public double getLado3() {
        return lado3;
    }

    public String getColorRelleno() {
        return colorRelleno;
    }

    public String getColorLinea() {
        return colorLinea;
    }
    
    // SETTERS

    public void setLado1(double lado1) {
        this.lado1 = lado1;
    }

    public void setLado2(double lado2) {
        this.lado2 = lado2;
    }

    public void setLado3(double lado3) {
        this.lado3 = lado3;
    }

    public void setColorRelleno(String colorRelleno) {
        this.colorRelleno = colorRelleno;
    }

    public void setColorLinea(String colorLinea) {
        this.colorLinea = colorLinea;
    }

    public double calcularPerimetro() {
        return lado1 + lado2 + lado3;
    }
    
    public double calcularArea() {
        double area = calcularPerimetro() / 2;
        return Math.sqrt(area*(area-lado1)*(area-lado2)*(area-lado3));
    }
    
}
