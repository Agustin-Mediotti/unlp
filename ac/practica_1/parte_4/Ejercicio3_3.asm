; Autor: Agustin Mediotti
; Fecha: 04-09-2025

ORG 1000h
            C   db 'C'

ORG 3000h
; Subrutina que convierte un caracter a letra minuscula
; Recibe:
;   AL: caracter a convertir
; Devuelve:
;   AL: caracter convertido a minuscula
A_MINUS:    OR al, 00100000b
            RET

ORG 2000h
            MOV al, C
            CALL A_MINUS
            INT 0
END