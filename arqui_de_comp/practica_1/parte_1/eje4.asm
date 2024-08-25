ORG 500h
        mensaje db "Hola Mundo"
        fin db 0

ORG 3000h
;  Subrutina a_minus
; ENTRADA:
;  cl, caracter ASCII en nayusculas
; SALIDA:
;  cl: caracter ASCII en minuscula
a_minus:
      OR cl, 00100000b  ; mascara 20h
      RET

ORG 2500h
;  Subrutina iter_str
; ENTRADA:
;  bx, dir del mensaje
;  dx, dir de la ultima posicion del mensaje en memoria
iter_str:
      MOV cl, [bx]
      CMP cl, 61h   ; minuscula?
      JNC sig
      CALL a_minus
      MOV [bx], cl 

sig:  ADD bx, 1     ; siguiente
      CMP bx, dx
      JNZ iter_str  ; ultimo?
      RET

ORG 2000h
        MOV bx, offset mensaje
        MOV dx, offset fin
        CALL iter_str
        int 0
END
