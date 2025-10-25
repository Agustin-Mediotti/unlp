# Ejercicio 11 - Orientacion a Objetos

![alt text](image.png)

## Tarea B:

1. *"Cuenta con ganchos"* se refiere a que la clase *Cuenta* tiene dos clases que heredan el comportamiento en común.
2. En las implementaciones de los métodos **extraer()** y **transferirACuenta()**, **this** representa la clase que hace la llamada al mensaje. Puede ser cualquiera de las clases que implementan
3. Son de visibilidad *"protegido"* porque son mensajes que vamos a extender en las clases que extiendan de la clase Cuenta
4. Si, se puede transferir de una caja de ahorro a una cuenta corriente y viseversa porque ambas clases heredan la misma funcionalidad de la clase padre y la adaptan a su particularidad de dominio
5. En Java, un método abstracto se declara como *protected abstract tipo nombreMetodo();* y es obligatorio implementarlo para todas las clases que extiendan el comportamiento de esa clase. Si una subclase no implementa un método abstracto que hereda, el compilador dice *The type must implement the inherited abstract method*