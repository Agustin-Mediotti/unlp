; Programa que verifica si un caracter es una letra mayuscula
; Si es una letra mayuscula, almacena 0FFh en RES
; Si no, almacena 00h en RES

; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
      C   db 'C'
      RES db ?


ORG 2000h
      MOV RES, 0
      MOV bl, C
      CMP bl, 41h
      JC  FIN
      CMP bl, 5Ah
      JNC FIN
      MOV RES, 0FFh
FIN:  INT 0
END
