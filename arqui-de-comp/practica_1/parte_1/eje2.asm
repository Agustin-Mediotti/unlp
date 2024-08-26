ORG 500h
           c db 'C'
           res db 0

ORG 3000h
;  Subrutina es_mayus
;  ENTRADA:
;    bl: Caracter
;    ah: Resultado de la operacion
;  SALIDA:
;    ah: Resultado de la operacion
es_mayus:  CMP bl, 41h
           JC no_es
           CMP bl, 54h
           JNC no_es
           MOV bh, 0FFh
           JMP fin
no_es:     MOV bh, 0
fin:       RET

ORG 2000h
           MOV bl, c
           MOV bh, res
           CALL es_mayus
           int 0
END
