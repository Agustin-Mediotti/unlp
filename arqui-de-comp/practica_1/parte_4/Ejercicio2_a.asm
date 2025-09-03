; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
        A db 8
        B db 5
        C db 4
        D db ?

ORG 3000h
CALC:   MOV dl, al
        ADD dl, ah
        SUB dl, cl
        RET
ORG 2000h
        MOV al, A
        MOV ah, B
        MOV cl, C
        CALL CALC
        MOV D, dl
        HLT
END