; Autor: Agustin Mediotti
; Fecha: 05-09-2025

ORG 1000h
        A   db  2
        B   db  8
        RES dw  ?

ORG 3000h
; Multiplica dos numeros de 8 bits
; Recibe:
;   [AX]: direccion del primer numero
;   [BX]: direccion del segundo numero
; Devuelve:
;   AX: resultado de la multiplicacion por valor
MUL:    PUSH bx
        PUSH dx
        PUSH cx
        MOV cx, [bx]
        MOV bx, ax
        MOV al, [bx]
        MOV ah, cl
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
        MOV ax, offset A
        MOV bx, offset B
        CALL MUL
        MOV RES, ax
        INT 0
END