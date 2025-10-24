|  Instruccion   | Valor Registro SP | Pila  |
| :------------: | :---------------: | :---: |
| **ORG 3000h**  |        ---        |       |
| rut: mov bx, 3 |       7FFEh       | 2004h |
|      ret       |       8000h       | 2004h |
| **ORG 2000h**  |        ---        |       |
|    call rut    |       7FFEh       | 2000h |
|   add cx, 5    |       8000h       | 2000h |
|    call rut    |       7FFEh       | 2004h |
|      hlt       |       8000h       | 2004h |
|    **END**     |        ---        |       |
> Estado de pila en ultimo **call rut**

a. Al ejecutarse la instruccion **call rut**, se hace un **push** de la dirección de retorno.

b. Al ejecutarse la instrucción **ret**, se hace un **pop** del stack al **IP** para luego saltar a la dirección.
