; Acceso con contraseña con intentos limitados
; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
      TRIES    db  5
      MSJ_LOG  db  "Por favor, ingrese la contraseña"
      SUCCESS  db  0Ch, "Acceso Permitido"
      DENIED   db  0Ch, "Acceso Denegado"
      FAILED   db  0Ch, "Acceso Bloqueado"
      PSW      db  "H4CK"
      INPUT    db  ?

ORG 2000h
      MOV bx, offset MSJ_LOG
      MOV al, offset SUCCESS - offset MSJ_LOG
TRY:  INT 7
      MOV al, 4
      MOV bx, offset INPUT
LOOP: INT 6
      INC bx
      DEC al
      JNZ LOOP
      MOV bx, offset PSW
      MOV cl, 4
NEXT: MOV al, [BX]
      CMP al, [BX + 4]
      JNZ ERR
      INC bx
      DEC cl
      JNZ NEXT
      MOV bx, offset SUCCESS
      MOV al, offset DENIED - offset SUCCESS
      JMP FIN
ERR:  MOV bx, offset DENIED
      MOV al, offset FAILED - offset DENIED
      DEC TRIES
      JNZ TRY
      MOV bx, offset FAILED
      MOV al, offset PSW - offset FAILED
FIN:  INT 7
      INT 0
END
