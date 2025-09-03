; Programa que cuenta la cantidad de letras 'a' en una cadena
; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
        str     db "Arquitectura de Computadoras"
        cant    db 0

ORG 2000h
        MOV bx, offset str
        MOV AL, offset cant - offset str
LOOP:   CMP byte ptr [BX], 'a'
        JNZ NEXT
        INC cant
NEXT:   INC bx
        DEC al
        JNZ LOOP
        INT 0
END