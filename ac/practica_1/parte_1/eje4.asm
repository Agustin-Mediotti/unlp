; Programa: Convierte string a minusculas
; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
      MENSAJE  db  "Hola, Buenas Tardes"
      FIN      db  ?

ORG 2000h
      MOV bx, offset MENSAJE
      MOV al, offset FIN - offset MENSAJE
NEXT: OR byte ptr [bx], 00100000b
      INC bx
      DEC al
      JNZ NEXT
      INT 0
END
