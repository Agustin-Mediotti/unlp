ORG 2750H
;  Subrutina print
;  Entrada:
;    bx: dir del char
;    al: cant de caracteres a imprimir
PRINT: INT 7
       CMP BYTE PTR [bx], 7EH  ; Ultimo caracter?
       JZ FIN
       INC CHAR
       JMP PRINT
FIN:   RET

ORG 500H
;  Primer caracter de la tabla ASCII + LF
CHAR DB 20H, 0AH

ORG 2000H
      MOV bx, offset CHAR
      MOV al, 2
      CALL PRINT
      INT 0
END