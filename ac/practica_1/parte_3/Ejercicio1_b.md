|    Instruccion    | Valor Registro SP |
| :---------------: | :---------------: |
|   **ORG 3000h**   |        ---        |
| rutina: mov bx, 3 |       7FFCh       |
|        ret        |       7FFEh       |
|   **ORG 2000h**   |        ---        |
|      push ax      |       7FFEh       |
|    call rutina    |       7FFCh       |
|      pop bx       |       8000h       |
|        hlt        |       8000h       |
|      **END**      |        ---        |
