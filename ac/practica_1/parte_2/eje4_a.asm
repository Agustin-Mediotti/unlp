; Mostrar letras 'A' a 'Z' en pantalla
; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
      ;  Caracter letra 'A' + salto de linea
      A db 41h, 0Ah

ORG 2000H
      MOV bx, offset A
      MOV al, 2
      MOV cl, 26
PRNT: INT 7
      INC A
      DEC cl
      JNZ PRNT
      INT 0
END
