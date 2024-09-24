HAND_DATA  EQU 40h
HAND_STATE EQU 41h


ORG 1000h
MSJ DB "INGENIERIA E INFORMATICA"
FIN DB ?

ORG 2000h
; Configuro el Handshake para el polling
  IN AL, HAND_STATE    ; Tomo estado actual
  AND AL, 7Fh          ; 7Fh = 01111111b
  OUT HAND_STATE, AL   ; Estado = 0xxxxxxx
; Recorro el mensaje y lo envio caracter
; por caracter hacia la impresora
  MOV BX, OFFSET MSJ    ; Direccion del mensaje
POLL: IN AL, HAND_STATE ; Tomo caracter actual
      AND AL, 1         ; Checkeo primer bit
      JNZ POLL          ; Mientras sea 1 sigo
      MOV AL, [BX]      ; Tomo el caracter
      OUT HAND_DATA, AL ; Lo envio al reg. datos
      INC BX            ; Avanzo sig. caracter
      CMP BX, OFFSET FIN; Checkeo si es el final
      JNZ POLL
      INT 0
END
