.data
    A:  .word   3
    B:  .word   2
    C:  .word   0

.text
    ld $t0, A($zero)
    ld $t1, B($zero)
    dadd $t2, $zero, $zero
    beqz $t0, fin
    slt $t2, $t1, $t0       # if t1 > t0 => t2 = 1 else t2 = 0
    bnez $t2, apor2         # t2 = 1 then jump apor2
    dadd $t2, $t1, $zero    # t2 = B
    j fin
apor2: daddi $t2, $zero, 2  # t2 = 2
       dmul $t2, $t0, $t2   # t2 = 2 * 3
fin:    sd $t2, C($zero)