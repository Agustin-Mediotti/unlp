; Convertir el caracter de mayúscula a minuscula
; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
      C   db 'C'

ORG 2000h
      OR C, 00100000b
      INT 0
END