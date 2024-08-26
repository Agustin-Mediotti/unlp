ORG 527H
PSW     DB "H4CK"
SUCCESS DB "Acceso Permitido"  ; 15b
DENIED  DB "Acceso Denegado"   ; 14b

ORG 652H
TRIES   DB 0
INPUT   DB ?

ORG 2670
;  Subrutina READ
;  Entrada:
;    bx: dir de lectura
;    al: cantidad de caracteres a leer
;    ah: intentos de lectura
ERR:   INC TRIES       
       CMP TRIES, 5       ; CHECK TRIES
       JZ DEN
       MOV AH, 0
       MOV BX, OFFSET INPUT
READ:  INT 6
       INC BX
       INC AH
       CMP AH, 4
       JNZ READ
       
    ;  CHECK PASS

       MOV BX, OFFSET PSW     ; <- BX es OFFSET PSW
       MOV DX, OFFSET INPUT   ; <- DX es OFFSET INPUT
NEXTC: MOV AL, [BX]           ; <- AL es PSW 
       CALL SWAP              ;  BX = INPUT, DX = PSW
       MOV AH, [BX]           ; <- AH es INPUT

       CMP AL, AH             ; Comparo el caracter
       JNZ ERR

       CALL SWAP              ;  BX = PSW, DX = INPUT
       MOV CX, OFFSET PSW + 3
       CMP BX, CX             ; ULTIMO CARACTER?
       JZ SUC
       INC BX
       INC DX
       JMP NEXTC
       
       RET

ORG 3500
;Subrutina Success
SUC:   MOV BX, OFFSET SUCCESS
       MOV AL, 1
       MOV CX, OFFSET SUCCESS + 15
       CALL PRINT
       RET

ORG 3600
;Subrutina Denied
DEN:   MOV BX, OFFSET DENIED
       MOV AL, 1
       MOV CX, OFFSET DENIED + 14
       CALL PRINT
       RET

ORG 4000
; Imprimir en pantalla
PRINT: INT 7
       CMP CX, BX       
       JZ FIN
       INC BX
       JMP PRINT
FIN:   RET

ORG 1290H
; Subrutina SWAP
; Intercambia los valores de dos registros
SWAP:  PUSH BX
       PUSH DX
       POP BX
       POP DX
       RET
; REGISTER SWAP BABY!

ORG 2000H
       MOV BX, OFFSET INPUT
       MOV AL, 1
       MOV AH, 0  ;  cant de caracteres de lectura
       CALL READ
       INT 0
END
