; for i = 1 to 1000 do
;   A[i] := B[i]+5;

.data
    base_B: .word 1,2,3,4,5,6,...,1000
    base_A: .space 1000

.text
        DADDI R2, R0, 1         ; variable i = 1 (en R2)
        DADDI R5, R0, 5         ; R5 = 5
        DADDI R10, R0, 1001     ; limite del FOR (en R10)
ciclo:  LD R1, base_B(R2)       ; R1 = B[i]
        DADD R1, R1, R5         ; R1 = B[i] + 5
        SD R1, base_A(R2)       ; A[i] = R1
        DADDI R2, R2, 1         ; i++
        BNE R2, R10, ciclo      ; i != 1001 => ir a ciclo
        HALT