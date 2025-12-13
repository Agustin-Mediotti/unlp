.data
A: .word   2
B: .word   8
C: .word   0

.text
ld $t1, A($t0)
ld $t2, B($t0)
dmul $t3, $t1, $t2
sd $t3, C($t0)
halt