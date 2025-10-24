HAND_DATA  EQU 40h
HAND_STATE EQU 41h

ORG 1000h
MSJ DB "INGENIERIA E INFORMATICA"
FIN DB ?

ORG 3000h
; Configuro el Handshake para el polling
ini_hs:  PUSH AX            ; Guardamos estado
         IN AL, HAND_STATE  ; Tomo estado actual
         AND AL, 7Fh        ; 7Fh = 01111111b
         OUT HAND_STATE, AL ; Estado = 0xxxxxxx
         POP AX             ; Restauramos estado
         RET                ; Retornamos
         
; Subrutina de Consulta de estado
poll:    PUSH AX            ; Guardamos estado
         IN AL, HAND_STATE  ; Tomamos el valor de HS
         AND AL, 1
         JNZ poll
         POP AX             ; Restauramos estado
         RET                ; Retornamos

; Subrutina Imprimir Caracter
; Recibe el Caracter a imprimir en AL
imp_car: PUSH BX            ; Guardamos estado
         CALL poll          ; Espero a que este lib.
         OUT HAND_DATA, AL  ; Envio car al bus
         POP BX             ; Restauramos estado
         RET                ; Retornamos

; Subrutina Imprimir Mensaje
; Recibe la direccion del mensaje en BX
imp_men: PUSH AX              ; Guardamos estado      
loop_i:  MOV AL, [BX]         ; Tomamos el caracter
         CALL imp_car
         INC BX               ; Siguiente caracter
         CMP BX, OFFSET MSJ+5 ; Checkeo ultimo car.
         JNZ loop_i
         POP AX               ; Restauramos estado
         RET                  ; Retornamos

; Subrutina Leer Mensaje
; Recibe la dir. donde se va a guardar en BX
leer_m:  PUSH BX              ; Guardamos estado 
loop_l:  INT 6                ; Leemos el caracter
         INC BX               ; Siguiente caracter
         CMP BX, OFFSET MSJ+5 ; Checkeo ultimo car.
         JNZ loop_l
         POP BX               ; Restauramos estado
         RET                  ; Retornamos

; Programa Principal
ORG 2000h
  CALL ini_hs
  MOV BX, OFFSET MSJ
  CALL leer_m
  CALL imp_men
  INT 0
END