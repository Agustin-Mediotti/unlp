.data
    str:     .asciiz "Hola"
    l_str:    .word   0

.text
    daddi $a0, $zero, str
    jal lo
    sd $v0, l_str($zero)
    halt
lo: daddi $v0, $zero, 0
l:  lbu $t0, 0($a0)
    beqz $t0, f
    daddi $v0, $v0, 1
    daddi $a0, $a0, 1
    j l
f:  jr $ra