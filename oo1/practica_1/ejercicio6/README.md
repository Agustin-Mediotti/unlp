# Ejercicio 6: Distribuidora Eléctrica

Una distribuidora eléctrica desea gestionar los consumos de sus usuarios para la emisión de facturas de cobro.
De cada usuario se conoce su nombre y domicilio. Se considera que cada usuario sólo puede tener un único domicilio en donde se registran los consumos.

Los consumos de los usuarios se dividen en dos componentes:
Consumo de energía activa: tiene un costo asociado para el usuario.  Se mide en kWh (kilowatt/hora).
Consumo de energía reactiva: no genera ningún costo para el usuario, es decir, se utiliza solamente para determinar si hay alguna bonificación.  Se mide en kVArh ( kilo voltio-amperio reactivo hora). 

Se cuenta con un cuadro tarifario que establece el precio del kWh para calcular el costo del consumo de energía activa. Este cuadro tarifario puede ser ajustado periódicamente según sea necesario (por ejemplo, para reflejar cambios en los costos).

Para emitir la factura de un cliente se tiene en cuenta solo su último consumo registrado. Los datos que debe contener la factura son los siguientes:
- El usuario a quien se está cobrando.
- La fecha de emisión.
- La bonificación, sí aplica.
- El monto final de la factura: se calcula restando la bonificación al costo del consumo:
- El costo del consumo se calcula multiplicando el consumo de energía activa por el precio del kWh proporcionado por el cuadro tarifario.
- Se calcula su factor de potencia para determinar si hay alguna bonificación aplicable. Si el factor de potencia estimado (fpe) del último consumo del usuario es mayor a 0.8, el usuario recibe una bonificación del 10%.

Tareas:
1. Realice la lista de conceptos candidatos.
2. Grafique el modelo de dominio usando UML.
3. Actualice el modelo de dominio incorporando los atributos a los conceptos
4. Agregue asociaciones entre conceptos, indicando para cada una de ellas la categoría a la que pertenece, de acuerdo a lo explicado en la teoría, y demás atributos, según sea necesario.
