PA EQU 30h
PB EQU 31h
CA EQU 32h
CB EQU 33h

ORG 2000h
  MOV AL, 00h
  OUT CB, AL
  MOV AL, 0FFh
  OUT CA, AL

LOOP: IN AL, PA
  OUT PB, AL
  JMP LOOP

  INT 0
END