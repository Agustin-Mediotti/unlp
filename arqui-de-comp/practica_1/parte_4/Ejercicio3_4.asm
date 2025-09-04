; Autor: Agustin Mediotti
; Fecha: 04-09-2025

ORG 1000h
      MENSAJE  db  "Hola, Buenas Tardes"
      FIN      db  ?

ORG 3000h
; Convierte una cadena de caracteres a minusculas
; Recibe:
;   BX: puntero a la cadena
;   AL: longitud de la cadena
; Devuelve:
;   La cadena convertida a minusculas en la misma posicion
STRING_A_MINUS: PUSH bx
                PUSH ax
LOOP:           OR byte ptr [bx], 00100000b
                INC bx
                DEC al
                JNZ LOOP
                POP ax
                POP bx
                RET

ORG 2000h
                MOV bx, offset MENSAJE
                MOV al, offset FIN - offset MENSAJE
                CALL STRING_A_MINUS
                INT 0
END
