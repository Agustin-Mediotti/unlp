.data
    A:  .word 10
    B:  .word 8
    C:  .word 0

.text
main:   ld r4, A(r0)         ; A en r4
        ld r5, B(r0)         ; B en r5
        dadd r3, r4, r5      ; r3 = r4 + r5
        sd r3, C(r0)         ; C = r3
        halt