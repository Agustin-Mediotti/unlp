ORG 2750H
;  Subrutina print
;  Entrada:
;    bx: dir del char
;    al: cant de caracteres a imprimir
PRINT: INT 7
       CMP BYTE PTR [bx], 7EH  ; Ultimo caracter?
       JZ FIN
       INC CHAR
       JMP print
FIN:   RET

ORG 500H
;  Primer caracter de la tabla ASCII
CHAR DB 20H

ORG 2000H
      MOV bx, offset CHAR
      MOV al, 1
      CALL PRINT
      INT 0
END