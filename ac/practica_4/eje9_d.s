.data
    inicio: .ascii "a"
    fin: .ascii "h"
    string: .ascii ""

.text
    lbu $t2, inicio($0) ; primer car
    daddi $t1, $0, 1    ; cant caracteres a imprimir
    daddi $t0, $0, $0   ; indice de desplazamiento

loop: sb $t2, string($t0)
      guardar: sb $t2, string($t0)
        daddi $t5, $t5, 1
        beq $t5, $t0 guardar

      dadd $t5, $0, $0
      j guardar ; incompleto xd