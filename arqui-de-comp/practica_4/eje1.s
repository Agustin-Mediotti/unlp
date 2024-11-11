.data
    A:  .word   3
    B:  .word   5

    S:  .word   0   # A + B
    P:  .word   0   # 2 + (A*B)
    D:  .word   0   # A² / B

.code
    ld $t0, A($zero)
    ld $t1, B($zero)

    dadd $t2, $t0, $t1
    sd $t2, S($zero)    # S = 8

    dmul $t3, $t0, $t1
    daddi $t3, $zero, 2
    sd $t3, P($zero)    # P = 17

    dmul $t4, $t0, $t0
    ddiv $t4, $t4, $t1
    sd $t4, D($zero)    # D = 1.8 (solo parte decimal)
    
    halt