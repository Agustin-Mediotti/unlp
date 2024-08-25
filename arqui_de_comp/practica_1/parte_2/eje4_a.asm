ORG 2750H
;  Subrutina print
;  Entrada:
;    bx: dir de los char
;    al: cant de caracteres a imprimir
print: INT 7
       CMP BYTE PTR [bx], 5Ah
       JZ fin
       INC A
       JMP print
fin:   RET

ORG 500H
;  Caracter letra 'A' + salto de linea
A DB 41h, 0AH

ORG 2000H
      MOV bx, offset A
      MOV al, 2
      CALL print
      INT 0
END
