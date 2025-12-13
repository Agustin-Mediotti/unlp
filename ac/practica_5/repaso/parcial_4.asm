.data
datos:          .word   12,42,51,23,17,37,-8,16,21,33
filtrados:      .word   0

.text
                daddi $a0,$0,datos # cargar ambos vectores en registro.
                daddi $a1,$0,filtrados # cargar cantidad de pares
                daddi $a2,$0,5
                jal filtrar
                halt
filtrar:        daddi $s1,$ra,0 # guardamos dirección de retorno en s1
sigo:           ld $t0,0($a0) # cargamos en t0 primer num
                jal en_rango # preguntamos si $t0 ta en rango
                beqz $v0,proximo_par # si no esta en rango próximo par
                daddi $t1,$t0,0 # guardamos en t1 el 12
                ld $t0,8($a0) # nos traemos a t0 el siguiente a 12
                jal en_rango # llamamos a en rango para 42
                bnez $v0,guardar_par
proximo_par:    daddi $a0,$a0,16
                daddi $a2,$a2,-1
                bnez $a2,sigo
                jr $s1
guardar_par:    sd $t1,0($a1)
                sd $t0,8($a1)
                daddi $a1,$a1,16
                j proximo_par
en_rango:       daddi $v0, $0, 0
                daddi $v1, $0, 0
                slti $v1,$t0,0
                bnez $v1, fin
                slti $v1,$t0,50
                beqz $v1,fin
                daddi $v0,$0,1
fin:            jr $ra