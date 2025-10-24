.data
    result: .word 0

.text
    daddi $a0, $0, 10
    daddi $a1, $0, 20
    jal sumar               ; se llama a la subrutina 'sumar'
    sd $v0, result($0)
    halt

sumar:  dadd $v0, $a0, $a1     ; subrutina 'sumar'
        jr $ra                 ; Retorna al punto donde se llamo la subrutina