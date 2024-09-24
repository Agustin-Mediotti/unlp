PA EQU 30h
PB EQU 31h
CA EQU 32h
CB EQU 33h

ORG 1000h
MSJ DB "ORGANIZACION Y ARQUITECTURA DE COMPUTADORAS"
FIN DB ?

ORG 3000h
; Subrutina de Configuracion puertos CA y CB
ini_pio: PUSH AX            ; Guardamos estado
         MOV AL, 11111101b  ; bit de Strobe salida 
         OUT CA, AL         ; y bit de busy entrada
         MOV AL, 0
         OUT CB, AL         ; Todos en salida
         POP AX             ; Restauramos estado
         RET                ; Retornamos
         
; Subrutina de Consulta de estado
poll:    PUSH AX            ; Guardamos estado
         MOV AL, PA         ; Tomamos el valor de PA
         AND AL, 1
         JNZ poll
         POP AX             ; Restauramos estado
         RET                ; Retornamos
         
; Subrutina Flanco Ascendente
f_asc:   PUSH AX            ; Guardamos estado
         MOV AL, PA         ; Tomamos el valor de PA
         AND AL, 0FDh       ; Fuerzo strobe a 0
         OUT PA, AL
         OR AL, 2           ; Fuerzo strobe a 1
         OUT PA, AL
         POP AX             ; Restauramos estado
         RET                ; Retornamos

; Subrutina Imprimir Caracter
; Recibe el Caracter a imprimir en AL
imp_car: PUSH BX            ; Guardamos estado
         CALL poll          ; Espero a que este lib.
         OUT PB, AL         ; Envio car al puerto
         CALL f_asc         ; Envio señal de strobe
         POP BX             ; Restauramos estado
         RET                ; Retornamos

; Subrutina Imprimir Mensaje
; Recibe la direccion del mensaje en BX
imp_men: PUSH AX            ; Guardamos estado      
loop:    MOV AL, [BX]       ; Tomamos el caracter
         CALL imp_car
         INC BX             ; Siguiente caracter
         CMP BX, OFFSET FIN ; Checkeo ultimo car.
         JNZ loop
         POP AX             ; Restauramos estado
         RET                ; Retornamos

; Programa Principal
ORG 2000h
  CALL ini_pio
  MOV BX, OFFSET MSJ
  CALL imp_men
  INT 0
END