.data
    A: .word 5
    B: .word 4
    C: .word 0

.text
    daddi $t0, $zero, 0
    ld $t1, A($t0)
    daddi $t0, $zero, 8
    ld $t2, A($t0)
    dadd $t3, $t2, $t1
    daddi $t0, $zero, 16
    sd $t3, A($t0)
halt