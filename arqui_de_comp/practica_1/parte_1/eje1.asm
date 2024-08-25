ORG 500h
           str db "Arquitectura de Computadoras"
           cant db 0

ORG 3000h
;  Subrutina iter_str
;  ENTRADA:
;    bx: dir del mensaje
;    ax: dir de var cant
;    cl: cant
;  SALIDA:
;    cl: cant
iter_str:  MOV dl, [bx]
           CMP dl, 61h   ; ASCII 'a'?
           JNZ sig
           ADD cl, 1     ; cant++
sig:       ADD bx, 1
           CMP ax, bx    ; ultimo caracter?
           JNZ iter_str
           RET

ORG 2000h
           MOV bx, offset str
           MOV ax, offset cant
           MOV cl, cant
           CALL iter_str
           MOV cant, cl
           int 0
END
