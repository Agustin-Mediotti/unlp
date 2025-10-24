EOI   EQU 20h
IMR   EQU 21h
INT2  EQU 26h

HAND_DATA  EQU 40h
HAND_STATE EQU 41h


ORG 1000h
MSJ DB "INGENIERIA E INFORMATICA"
FIN DB ?

ORG 3000h
; Recorro el mensaje y lo envio caracter
; por caracter hacia la impresora
PRINT: PUSH AX           ; Guardo el estado
       MOV AL, [BX]      ; Tomo el caracter
       OUT HAND_DATA, AL ; Lo envio al reg. datos
       INC BX            ; Avanzo sig. caracter
       MOV AL, 20h       ; Aviso al PIC
       OUT EOI, AL
       POP AX            ; Recupero el estado
       IRET              ; y retorno

ORG 2000h
; Configuro el vector de interrupciones ID = 9
  MOV AX, PRINT
  MOV BX, 36           ; 36 = 9 X 4
  MOV [BX], AX

  CLI

  ; Configuro el PIC
  MOV AL, 0FBh         ; 0FBh = 111111011b
  OUT IMR, AL          ; Solo Handshake habilitado

  MOV AL, 9            ; Envio el ID seleccionado
  OUT INT2, AL         ; al registro INT2

  MOV BX, OFFSET MSJ   ; Para recorrer el mensaje

  ; Configuro el Handshake para interrupciones
  IN AL, HAND_STATE    ; Tomo estado actual
  OR AL, 80h           ; 80h = 10000000b
  OUT HAND_STATE, AL   ; Estado = 1xxxxxxx

  STI

  ; Continua programa principal
POLL: CMP BX, OFFSET FIN
      JNZ POLL            ; Checkeamos fin mensaje

  ; Desactivo el Handshake por interrupcion
  IN AL, HAND_STATE       ; Tomo estado actual
  AND AL, 7Fh             ; 7Fh = 01111111b
  OUT HAND_STATE, AL      ; Estado = 0xxxxxxx
  INT 0

END
