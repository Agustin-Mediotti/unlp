# Arquitectura CISC y Compiladores

Fue motivado por el deseo de realziar máquinas que proporcionaran un mejor
soporte a HLL para reducir el salto semantico con el lenguaje máquina.

## Estudio de ejecucon de inst.

- las sentencias de asignacion predominan, lo que indica que el movimiento de datos tiene **gran importancia**
- cuantas inst. tienen acceso a memoria?

Las sentencias de tipo lazo y llamada a procedimientos son sentecnias que en ejecucion tienen una aparicien dimanica muy baja.

## Operandos

- Hay que optimizar principalmente el acceso a variables escalares locales (75% del acceso a memoria)

## Procedimientos: argunmentos y variables locales

- caracterizar elcomportmamiento de los procedimietos.
- el 98% pasaba menos de 6 args 
- es raro el anidamiento, esto confirma el principio de localidad
- llamadas a procedimientos generan el 70% de las instrucciones.

### Conclusion

para reducir el acceso a memoria, se utilizarian mas registros.
hay limitacion en la profundidad de llamadas.

## Ventana de registro

registrso temporales que se solapan

## ventana de instrucciones

...

## Banco de registros amplio vs Cache

banco de registros amplio:
todos los datos escalares locales


## Procesadores Superescalares

La dependencia de datos se resuelven por hardware especializado. Cuando entra el IF, la dep. ya esta resuelta.

**distribuida** es cuand a sido enviada a una estacion de reserva.


## Politicas de emisión de instrucciones y ejecucion paralela

La emision es alineada si no puede introducirse nuevas instrucciones en la ventana hast que no esta
totalmente vacia.

en la emision ordenada se respeta el orden de las instrucciones se han introdcido en la ventan de isntrucciones
y coincide con el orden de las instrucciones en el programa
