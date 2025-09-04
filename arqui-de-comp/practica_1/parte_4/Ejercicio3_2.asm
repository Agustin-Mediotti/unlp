; Autor: Agustin Mediotti
; Fecha: 04-09-2025

ORG 1000h
            C   db 'z'

ORG 3000h
; Subrutina que verifica si un caracter es una letra mayuscula
; Recibe:
;   AL: caracter a verificar
; Devuelve:
;   AH: 0FFh si es mayuscula, 00h si no
ES_MAYUS:   MOV ah, 0
            CMP al, 41h
            JC  FIN
            CMP al, 5Ah
            JZ  ES
            JNC FIN
ES:         MOV ah, 0FFh
FIN:        RET

ORG 2000h
            MOV al, C
            CALL ES_MAYUS
            INT 0
END
