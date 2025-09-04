; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
            str    db "Arquitectura de Computadoras"
            fin    db 0

ORG 3000h
; Subrutina que cuenta la cantidad de apariciones de un caracter en una cadena
; Recibe:
;   BX: puntero a la cadena
;   AL: longitud de la cadena
;   AH: caracter a buscar
; Devuelve:
;   CL: cantidad de apariciones del caracter
CONTAR_CAR: XOR cl, cl
            PUSH bx
            PUSH ax
LOOP:       CMP byte ptr [bx], ah
            JNZ NEXT
            INC cl
NEXT:       INC bx
            DEC al
            JNZ LOOP
            POP ax
            POP bx
            RET

ORG 2000h
            MOV bx, offset str
            MOV al, offset fin - offset str
            MOV ah, 'a'
            CALL CONTAR_CAR
            INT 0
END