; Autor: Agustin Mediotti
; Fecha: 03-09-2025

ORG 1000h
          A db 8
          B db 5
          C db 4
          D db ?

ORG 3500
SWAP:     PUSH bx
          PUSH ax
          POP bx
          POP ax
          RET
  
ORG 3000h
CALC:     PUSH bx
          PUSH ax
          MOV bx, sp
          ADD bx, 10
          MOV ax, [bx]
          CALL SWAP
          MOV dx, [bx]
          CALL SWAP
          SUB bx, 2
          MOV ax, [bx]
          CALL SWAP
          ADD dx, [bx]
          CALL SWAP
          SUB bx, 2
          MOV ax, [bx]
          CALL SWAP
          SUB dx, [bx]
          CALL SWAP
          POP ax
          POP bx
          RET
ORG 2000h
        MOV ax, offset A
        PUSH ax
        MOV ax, offset B
        PUSH ax
        MOV ax, offset C
        PUSH ax
        CALL CALC
        MOV D, dl
        POP ax
        POP ax
        POP ax
        HLT
END
