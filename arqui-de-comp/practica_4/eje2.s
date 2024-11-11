.data
    A:  .word   5
    B:  .word   4
    C:  .word   0

.text
    ld $t0, A($zero)
    ld $t1, B($zero)

    beqz $t0, eq        # if A equals 0 => C equals 0
    
    slt $t3, $t0, $t1   # A > B = 0
    beqz $t3, nt

    daddi $t2, $zero, 2
    dmul $t4, $t0, $t2
    sd $t4, C($zero)    # C = A * 2
    j fn

nt: sd $t1, C($zero)    # C = B
    j fn

eq: sd $zero, C($zero)  # C = 0

fn: halt