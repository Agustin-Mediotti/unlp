ORG 500h
        letra_mayus db 'A'

ORG 3000h
;  Subrutina a_minus
; ENTRADA:
;  cl, caracter ASCII en nayusculas
; SALIDA:
;  cl: caracter ASCII en minuscula
a_minus:
      OR cl, 00100000b  ; mascara 20h
      RET

ORG 2000h
        MOV cl, letra_mayus
        CALL a_minus
        int 0
END
