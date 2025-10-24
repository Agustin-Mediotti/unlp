; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
        A db 8
        B db 5
        C db 4
        D db ?

ORG 3000h
CALC:   PUSH bx
        MOV bx, sp
        ADD bx, 8
        MOV dl, [bx]
        SUB bx, 2
        ADD dl, [bx]
        SUB bx, 2
        SUB dl, [bx]
        POP bx
        RET
ORG 2000h
        MOV al, A
        PUSH ax
        MOV al, B
        PUSH ax
        MOV al, C
        PUSH ax
        CALL CALC
        MOV D, dl
        POP ax
        POP ax
        POP ax
        HLT
END
