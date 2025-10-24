; Autor: Agustin Mediotti
; Fecha: 05-09-2025

ORG 1000h
        A   db  2
        B   db  8
        RES dw  ?

ORG 3000h
; Multiplica dos numeros de 8 bits
; Recibe:
;   AL: primer numero
;   AH: segundo numero
; Devuelve:
;   AX: resultado de la multiplicacion por refencia
MUL:    PUSH bx
        PUSH dx
        PUSH cx
        CMP al, 0
        JZ ZERO
        CMP ah, 0
        JZ ZERO
        MOV dl, ah
        XOR dh, dh
        XOR ah, ah
        XOR cx, cx
LOOP:   ADD cx, dx
        DEC ax
        JNZ LOOP
        MOV ax, cx
        JMP BACK
ZERO:   XOR ax, ax
BACK:   POP cx
        POP dx
        POP bx
        MOV RES, ax
        MOV ax, offset RES
        RET

ORG 2000h
        MOV al, A
        MOV ah, B
        CALL MUL
        INT 0
END