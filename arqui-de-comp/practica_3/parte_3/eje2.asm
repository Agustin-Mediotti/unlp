PA EQU 30h
PB EQU 31h
CA EQU 32h
CB EQU 33h

IMR EQU 21h
EOI EQU 20h
INT0 EQU 24h

N_F10 EQU 10

ORG 40
DW RUT_F10

ORG 1000h
ESTADO DB 0

ORG 3200h
RUT_F10: PUSH AX
         XOR ESTADO, 0FFh  
         MOV AL, ESTADO
         OUT PB, AL
         MOV AL, EOI
         OUT EOI, AL
         POP AX
         IRET

ORG 2000h
  CLI
    MOV AL, 0   ; puerto de datos todo salida
    OUT CB, AL
    OUT PB, AL
    MOV AL, 0FEh
    OUT IMR, AL
    MOV AL, N_F10
    OUT INT0, AL
  STI
  
  LOOP:JMP LOOP
END