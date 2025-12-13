# Calculo de CPI

Simular la ejecución del siguiente programa manualmente (sin usar el simulador), dibujando el cauce como lo 
hace el simulador, y  calculando la cantidad de instrucciones, ciclos, y CPIs. 

|                            |        |        |        |         |        |        |        |        |        | 
| -------------------------- | ------ | ------ | ------ | ------- | ------ | ------ | ------ | ------ | ------ |
|      `daddi $t2,$t3,5`     |   IF   |   ID   |   EX   |   MEM   |   WD   |        |        |        |        |
|      `dsub $t4,$t3,$t5`    |        |   IF   |   ID   |   EX    |   MEM  |   WD   |        |        |        |
|      `xor $t6,$t3,$t5`     |        |        |   IF   |   ID    |   EX   |   MEM  |   WD   |        |        |
|      `nop`                 |        |        |        |   IF    |   ID   |   EX   |   MEM  |   WD   |        |
|      `halt`                |        |        |        |         |   IF   |   ID   |   EX   |   MEM  |   WD   |
| -------------------------- | ------ | ------ | ------ | ------- | ------ | ------ | ------ | ------ | ------ |


|                            |             | 
| -------------------------  | ----------- |
| **Cantidad Instrucciones** |      5      |
| **Cantidad de Ciclos**     | 5 + 4 = 9   |
|         **CPI**            | 9 / 5 = 1.8 |
| -------------------------  | ----------- |


## Responder: 

-  La fórmula de CPI presentada anteriormente ¿se cumple para este programa? 
-  La instrucción `HALT` solo detiene la ejecución del programa ¿se cuenta en el cálculo del CPI? 
-  El  simulador  considera  a  la  instrucción  `NOP`  para  calcular  el  CPI.  No  obstante,  dicha 
instrucción  no  realiza  ninguna  tarea.  Agregar  20  instrucciones  `NOP`  más,  y  calcular 
nuevamente el CPI. ¿Qué valor toma? En el caso de que sea menor, ¿eso quiere decir que el 
programa es más eficiente?

1. Si, la fórmula `CPI = I + A + 4 / I` se cumple.
2. La instrucción `HALT` se cuenta en el cálculo del CPI.
3. Con 20 nuevas instrucciones `NOP`, el CPI queda en 1.1 pero el programa no es más eficiente, ya que ahora tiene una mayor cantidad de ciclos.