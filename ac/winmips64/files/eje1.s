.data
    A:  .word   2
    B:  .word   3
    S:  .word   0   # 2 + 3 = 5
    P:  .word   0   # 2 * 3 = 6
    D:  .word   0   # 2² / 3 = 1

.code
    ld $t0, A($zero)
    ld $t1, B($zero)
    dadd $t2, $t0, $t1  # t2 = A + B
    sd $t2, S($zero)    # S = t2

    dmul $t2, $t0, $t1  # t2 = 2 * 3
    daddi $t3, $t2, 2   # t3 = 5 + 2
    sd $t3, P($zero)    # P = 7
    dmul $t2, $t0, $t0  # t2 = 2 * 2
    ddiv $t2, $t2, $t1  # t2 = 4 / 3
    sd $tw, D($zero)    # D = 1
    halt