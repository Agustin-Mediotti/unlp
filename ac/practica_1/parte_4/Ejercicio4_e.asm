; Autor: Agustin Mediotti
; Fecha: 05-09-2025

ORG 1000h
        A   db  2
        B   db  8
        RES dw  ?

ORG 3000h
; Multiplica dos numeros de 8 bits
; Recibe:
;   [SP+8]: primer numero por refencia
;   [SP+10]: segundo numero por refencia
; Devuelve:
;   AX: resultado de la multiplicacion por valor
MUL:    PUSH bx
        PUSH dx
        PUSH cx
        MOV bx, sp
        ADD bx, 10
        MOV bx, [bx]
        MOV al, [bx]
        MOV bx, sp
        ADD bx, 8
        MOV bx, [bx]
        MOV ah, [bx]
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
        RET

ORG 2000h
        MOV bx, offset A
        PUSH bx
        MOV bx, offset B
        PUSH bx
        CALL MUL
        POP bx
        POP bx
        MOV RES, ax
        INT 0
END